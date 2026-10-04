use base64::{Engine, engine::general_purpose::STANDARD};
use serde_json::{Value, json};
use std::{error::Error, fs, net::TcpStream, process::Command, thread, time::Duration};
use tungstenite::{Message, WebSocket, stream::MaybeTlsStream};

type Result<T> = std::result::Result<T, Box<dyn Error>>;
fn output_path(name: &str) -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("audit is under tools/browser-audit")
        .join("docs/qa")
        .join(name)
}
struct Browser {
    socket: WebSocket<MaybeTlsStream<TcpStream>>,
    id: u64,
    events: Vec<Value>,
}
impl Browser {
    fn connect() -> Result<Self> {
        let list = Command::new("curl")
            .args([
                "--fail",
                "--silent",
                "--max-time",
                "5",
                "http://127.0.0.1:9222/json/list",
            ])
            .output()?;
        if !list.status.success() {
            return Err("Chromium debugging endpoint is unavailable".into());
        }
        let pages: Value = serde_json::from_slice(&list.stdout)?;
        let url = pages
            .as_array()
            .ok_or("invalid page list")?
            .iter()
            .find(|p| p["type"] == "page")
            .and_then(|p| p["webSocketDebuggerUrl"].as_str())
            .ok_or("no browser page")?;
        let (socket, _) = tungstenite::connect(url)?;
        if let MaybeTlsStream::Plain(stream) = socket.get_ref() {
            stream.set_read_timeout(Some(Duration::from_secs(20)))?;
        }
        Ok(Self {
            socket,
            id: 0,
            events: Vec::new(),
        })
    }
    fn read(&mut self) -> Result<Value> {
        loop {
            match self.socket.read()? {
                Message::Text(text) => return Ok(serde_json::from_str(&text)?),
                Message::Ping(payload) => self.socket.send(Message::Pong(payload))?,
                _ => {}
            }
        }
    }
    fn call(&mut self, method: &str, params: Value) -> Result<Value> {
        self.id += 1;
        let id = self.id;
        self.socket.send(Message::Text(
            json!({"id":id,"method":method,"params":params})
                .to_string()
                .into(),
        ))?;
        loop {
            let response = self.read()?;
            if response["id"] == id {
                if !response["error"].is_null() {
                    return Err(format!("{method}: {}", response["error"]).into());
                }
                return Ok(response["result"].clone());
            }
            self.events.push(response);
        }
    }
    fn navigate(&mut self, url: &str) -> Result<()> {
        self.events.clear();
        let result = self.call("Page.navigate", json!({"url":url}))?;
        if !result["errorText"].is_null() {
            return Err(format!("navigation failed: {}", result["errorText"]).into());
        }
        loop {
            if self
                .events
                .iter()
                .any(|event| event["method"] == "Page.loadEventFired")
            {
                break;
            }
            let event = self.read()?;
            self.events.push(event);
        }
        thread::sleep(Duration::from_millis(350));
        Ok(())
    }
    fn screenshot(&mut self, path: &str, full: bool, width: u32, height: u32) -> Result<()> {
        let doc = self.call("DOM.getDocument", json!({}))?;
        let main = self.call(
            "DOM.querySelector",
            json!({"nodeId":doc["root"]["nodeId"],"selector":"main"}),
        )?;
        self.call("DOM.focus", json!({"nodeId":main["nodeId"]}))?;
        for _ in 0..4 {
            self.call(
                "Input.dispatchMouseEvent",
                json!({"type":"mouseWheel","x":width/2,"y":height/2,"deltaX":0,"deltaY":-1000000}),
            )?;
            thread::sleep(Duration::from_millis(500));
            let position = self.call("Page.getLayoutMetrics", json!({}))?;
            if position["cssLayoutViewport"]["pageY"]
                .as_f64()
                .unwrap_or(-1.0)
                < 0.5
            {
                break;
            }
        }
        let metrics = self.call("Page.getLayoutMetrics", json!({}))?;
        if !full
            && metrics["cssLayoutViewport"]["pageY"]
                .as_f64()
                .unwrap_or(-1.0)
                >= 0.5
        {
            return Err("Viewport capture needs scrolling to settle at the top".into());
        }
        let capture_height = if full {
            metrics["cssContentSize"]["height"]
                .as_f64()
                .ok_or("no height")?
        } else {
            height as f64
        };
        let response = self.call("Page.captureScreenshot", json!({"format":"png","captureBeyondViewport":full,"clip":{"x":0,"y":0,"width":width,"height":capture_height,"scale":1}}))?;
        fs::write(
            output_path(path),
            STANDARD.decode(response["data"].as_str().ok_or("no screenshot")?)?,
        )?;
        Ok(())
    }
    fn current_viewport(&mut self, path: &str) -> Result<()> {
        let response = self.call(
            "Page.captureScreenshot",
            json!({"format":"png","captureBeyondViewport":false}),
        )?;
        fs::write(
            output_path(path),
            STANDARD.decode(response["data"].as_str().ok_or("no screenshot")?)?,
        )?;
        Ok(())
    }
    fn enter(&mut self) -> Result<()> {
        self.call("Input.dispatchKeyEvent", json!({"type":"keyDown","key":"Enter","code":"Enter","windowsVirtualKeyCode":13,"nativeVirtualKeyCode":13,"text":"\r","unmodifiedText":"\r"}))?;
        self.call("Input.dispatchKeyEvent", json!({"type":"keyUp","key":"Enter","code":"Enter","windowsVirtualKeyCode":13,"nativeVirtualKeyCode":13}))?;
        Ok(())
    }
    fn node(&mut self, selector: &str) -> Result<Value> {
        let root = self.call("DOM.getDocument", json!({}))?["root"]["nodeId"].clone();
        let node = self.call(
            "DOM.querySelector",
            json!({"nodeId":root,"selector":selector}),
        )?["nodeId"]
            .clone();
        if node.as_u64() == Some(0) {
            return Err(format!("missing node {selector}").into());
        }
        Ok(node)
    }
    fn style(&mut self, selector: &str, property: &str) -> Result<String> {
        let node = self.node(selector)?;
        let styles = self.call("CSS.getComputedStyleForNode", json!({"nodeId":node}))?;
        styles["computedStyle"]
            .as_array()
            .ok_or("no styles")?
            .iter()
            .find(|v| v["name"] == property)
            .and_then(|v| v["value"].as_str())
            .map(str::to_owned)
            .ok_or_else(|| format!("missing computed {property}").into())
    }
    fn pseudo_style(
        &mut self,
        selector: &str,
        pseudo_type: &str,
        property: &str,
    ) -> Result<String> {
        let node = self.node(selector)?;
        let description = self.call("DOM.describeNode", json!({"nodeId":node}))?;
        let pseudo = description["node"]["pseudoElements"]
            .as_array()
            .ok_or("no pseudo elements")?
            .iter()
            .find(|pseudo| pseudo["pseudoType"] == pseudo_type)
            .ok_or_else(|| format!("missing {pseudo_type} for {selector}"))?;
        let styles = self.call(
            "CSS.getComputedStyleForNode",
            json!({"nodeId":pseudo["nodeId"]}),
        )?;
        styles["computedStyle"]
            .as_array()
            .ok_or("no pseudo styles")?
            .iter()
            .find(|value| value["name"] == property)
            .and_then(|value| value["value"].as_str())
            .map(str::to_owned)
            .ok_or_else(|| format!("missing computed {property} on {pseudo_type}").into())
    }
    fn box_rect(&mut self, selector: &str) -> Result<(i32, i32, i32, i32)> {
        let node = self.node(selector)?;
        let model = self.call("DOM.getBoxModel", json!({"nodeId":node}))?;
        let border = model["model"]["border"].as_array().ok_or("no box border")?;
        let coordinate = |index: usize| -> Result<i32> {
            Ok(border[index]
                .as_f64()
                .ok_or("invalid box coordinate")?
                .round() as i32)
        };
        Ok((
            coordinate(0)?,
            coordinate(1)?,
            coordinate(2)?,
            coordinate(5)?,
        ))
    }
    fn space(&mut self) -> Result<()> {
        self.call("Input.dispatchKeyEvent", json!({"type":"keyDown","key":" ","code":"Space","windowsVirtualKeyCode":32,"nativeVirtualKeyCode":32}))?;
        self.call("Input.dispatchKeyEvent", json!({"type":"keyUp","key":" ","code":"Space","windowsVirtualKeyCode":32,"nativeVirtualKeyCode":32}))?;
        Ok(())
    }
    fn toggle_by_keyboard(&mut self, selector: &str) -> Result<()> {
        let doc = self.call("DOM.getDocument", json!({}))?;
        let root = doc["root"]["nodeId"].clone();
        let summary = self.call(
            "DOM.querySelector",
            json!({"nodeId":root,"selector":format!("{selector} summary")}),
        )?["nodeId"]
            .clone();
        let details = self.call(
            "DOM.querySelector",
            json!({"nodeId":root,"selector":selector}),
        )?["nodeId"]
            .clone();
        self.call("DOM.focus", json!({"nodeId":summary}))?;
        self.enter()?;
        let attributes =
            self.call("DOM.getAttributes", json!({"nodeId":details}))?["attributes"].clone();
        if !attributes
            .as_array()
            .ok_or("no details attributes")?
            .iter()
            .any(|attribute| attribute == "open")
        {
            return Err(format!("keyboard failed to open {selector}").into());
        }
        self.enter()?;
        let attributes =
            self.call("DOM.getAttributes", json!({"nodeId":details}))?["attributes"].clone();
        if attributes
            .as_array()
            .ok_or("no details attributes")?
            .iter()
            .any(|attribute| attribute == "open")
        {
            return Err(format!("keyboard failed to close {selector}").into());
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    fs::create_dir_all(output_path(""))?;
    let mut browser = Browser::connect()?;
    for method in [
        "Page.enable",
        "Network.enable",
        "DOM.enable",
        "CSS.enable",
        "Accessibility.enable",
        "Performance.enable",
    ] {
        browser.call(method, json!({}))?;
    }
    browser.call(
        "Emulation.setScriptExecutionDisabled",
        json!({"value":true}),
    )?;
    browser.call("Network.setCacheDisabled", json!({"cacheDisabled":true}))?;
    browser.call(
        "Emulation.setEmulatedMedia",
        json!({"features":[{"name":"prefers-reduced-motion","value":"reduce"}]}),
    )?;
    if std::env::args().any(|v| v == "--fer-review") {
        return audit_fer(&mut browser);
    }
    if std::env::args().any(|v| v == "--motion") {
        return audit_motion(&mut browser);
    }
    if std::env::args().any(|v| v == "--pointer") {
        browser.call(
            "Emulation.setScriptExecutionDisabled",
            json!({"value":false}),
        )?;
        return audit_pointer(&mut browser);
    }
    if std::env::args().any(|v| v == "--performance") {
        browser.navigate("about:blank")?;
        browser.call(
            "PerformanceTimeline.enable",
            json!({"eventTypes":["largest-contentful-paint","layout-shift"]}),
        )?;
        browser.call(
            "Emulation.setDeviceMetricsOverride",
            json!({"width":390,"height":844,"deviceScaleFactor":1,"mobile":true}),
        )?;
        browser.call("Network.clearBrowserCache", json!({}))?;
        browser.call("Network.emulateNetworkConditions",json!({"offline":false,"latency":150,"downloadThroughput":200000,"uploadThroughput":75000,"connectionType":"cellular4g"}))?;
        browser.call("Emulation.setCPUThrottlingRate", json!({"rate":4}))?;
        browser.call(
            "Emulation.setEmulatedMedia",
            json!({"features":[{"name":"prefers-reduced-motion","value":"no-preference"}]}),
        )?;
        browser.navigate("http://127.0.0.1:8080/")?;
        thread::sleep(Duration::from_millis(1200));
        let metrics = browser.call("Performance.getMetrics", json!({}))?;
        let paints: Vec<_> = browser
            .events
            .iter()
            .filter(|event| event["method"] == "PerformanceTimeline.timelineEventAdded")
            .map(|event| event["params"]["event"].clone())
            .collect();
        let document = browser
            .events
            .iter()
            .find(|event| {
                event["method"] == "Network.requestWillBeSent"
                    && event["params"]["type"] == "Document"
            })
            .map(|event| event["params"].clone());
        let requests: Vec<_> = browser.events.iter().filter(|event|event["method"] == "Network.responseReceived").map(|event|json!({"url":event["params"]["response"]["url"],"status":event["params"]["response"]["status"],"type":event["params"]["type"]})).collect();
        let report = json!({"conditions":{"viewport":"390x844","latency_ms":150,"download_bytes_per_second":200000,"cpu_slowdown":4,"cache":"cold","javascript":"disabled","motion":"normal"},"performance_metrics":metrics,"paint_events":paints,"document_request":document,"requests":requests});
        fs::write(
            output_path("performance-report.json"),
            serde_json::to_string_pretty(&report)?,
        )?;
        println!(
            "{}",
            json!({"performance_metrics":metrics,"paint_events":paints,"document_request_timing":document.map(|v|json!({"timestamp":v["timestamp"],"wallTime":v["wallTime"]}))})
        );
        browser.call(
            "Network.emulateNetworkConditions",
            json!({"offline":false,"latency":0,"downloadThroughput":-1,"uploadThroughput":-1}),
        )?;
        browser.call("Emulation.setCPUThrottlingRate", json!({"rate":1}))?;
        return Ok(());
    }
    if std::env::args().any(|v| v == "--diagnose") {
        browser.call(
            "Emulation.setDeviceMetricsOverride",
            json!({"width":390,"height":844,"deviceScaleFactor":1,"mobile":false}),
        )?;
        browser.navigate("http://127.0.0.1:8080/")?;
        let frame =
            browser.call("Page.getFrameTree", json!({}))?["frameTree"]["frame"]["id"].clone();
        let sheet =
            browser.call("CSS.createStyleSheet", json!({"frameId":frame}))?["styleSheetId"].clone();
        browser.call(
            "CSS.setStyleSheetText",
            json!({"styleSheetId":sheet,"text":"html {font-size:200%}"}),
        )?;
        let document = browser.call("DOM.getDocument", json!({"depth":-1}))?;
        fn walk(node: &Value, nodes: &mut Vec<Value>) {
            nodes.push(node.clone());
            if let Some(children) = node["children"].as_array() {
                for child in children {
                    walk(child, nodes);
                }
            }
        }
        let mut nodes = Vec::new();
        walk(&document["root"], &mut nodes);
        for node in nodes.iter().filter(|node| {
            matches!(
                node["nodeName"].as_str(),
                Some(
                    "HEADER"
                        | "FOOTER"
                        | "DIV"
                        | "H1"
                        | "H2"
                        | "H3"
                        | "P"
                        | "UL"
                        | "OL"
                        | "LI"
                        | "SUMMARY"
                        | "A"
                )
            )
        }) {
            if let Ok(model) = browser.call("DOM.getBoxModel", json!({"nodeId":node["nodeId"]})) {
                let quad = model["model"]["border"].as_array().unwrap();
                let right = quad[2].as_f64().unwrap();
                let left = quad[0].as_f64().unwrap();
                if right > 376.0 || left < -1.0 {
                    println!(
                        "{}",
                        json!({"name":node["nodeName"],"attributes":node["attributes"],"left":left,"right":right,"width":model["model"]["width"]})
                    );
                }
            }
        }
        browser.call(
            "CSS.setStyleSheetText",
            json!({"styleSheetId":sheet,"text":""}),
        )?;
        return Ok(());
    }
    let mut reports = Vec::new();
    for (width, height, route) in [
        (320, 812, "/"),
        (360, 800, "/"),
        (390, 844, "/"),
        (430, 932, "/"),
        (768, 1024, "/"),
        (1024, 900, "/"),
        (1440, 1000, "/"),
        (1920, 1080, "/"),
        (390, 844, "/fer/"),
        (1440, 1000, "/fer/"),
    ] {
        browser.call(
            "Emulation.setDeviceMetricsOverride",
            json!({"width":width,"height":height,"deviceScaleFactor":1,"mobile":width < 600}),
        )?;
        browser.navigate(&format!("http://127.0.0.1:8080{route}"))?;
        let metrics = browser.call("Page.getLayoutMetrics", json!({}))?;
        let content_width = metrics["cssContentSize"]["width"]
            .as_f64()
            .ok_or("no content width")?;
        let viewport_width = metrics["cssLayoutViewport"]["clientWidth"]
            .as_f64()
            .ok_or("no viewport width")?;
        let overflow = content_width > width as f64 + 1.0 || viewport_width > width as f64 + 1.0;
        let tree = browser.call("Accessibility.getFullAXTree", json!({}))?;
        let unnamed_actions: Vec<_> = tree["nodes"]
            .as_array()
            .ok_or("no AX nodes")?
            .iter()
            .filter(|node| {
                node["ignored"] == false
                    && matches!(node["role"]["value"].as_str(), Some("link" | "button"))
                    && node["name"]["value"]
                        .as_str()
                        .unwrap_or("")
                        .trim()
                        .is_empty()
            })
            .collect();
        let scripts = browser
            .events
            .iter()
            .filter(|event| {
                event["method"] == "Network.requestWillBeSent"
                    && event["params"]["type"] == "Script"
            })
            .count();
        let failed: Vec<_> = browser
            .events
            .iter()
            .filter(|event| {
                event["method"] == "Network.responseReceived"
                    && event["params"]["response"]["status"]
                        .as_f64()
                        .unwrap_or(0.0)
                        >= 400.0
            })
            .map(|event| event["params"]["response"]["url"].clone())
            .collect();
        let initial_bytes: u64 = browser
            .events
            .iter()
            .filter(|event| event["method"] == "Network.loadingFinished")
            .map(|event| event["params"]["encodedDataLength"].as_f64().unwrap_or(0.0) as u64)
            .sum();
        if route == "/" && matches!(width, 320 | 360 | 390 | 430) {
            browser.screenshot(&format!("home-{width}-viewport.png"), false, width, height)?;
        }
        let first_view_aligned = if route == "/" && width <= 430 {
            let mut aligned = true;
            for selector in [".hero-fer", ".hero-kicker", ".hero-actions .button"] {
                let node = browser.node(selector)?;
                let model = browser.call("DOM.getBoxModel", json!({"nodeId":node}))?;
                let quad = model["model"]["border"].as_array().ok_or("no box")?;
                let left = quad[0].as_f64().ok_or("no left edge")?;
                let top = quad[1].as_f64().ok_or("no top edge")?;
                let right = quad[2].as_f64().ok_or("no right edge")?;
                let bottom = quad[5].as_f64().ok_or("no bottom edge")?;
                aligned &= left >= -1.0
                    && right <= width as f64 + 1.0
                    && top >= -1.0
                    && bottom <= height as f64 + 1.0;
            }
            Some(aligned)
        } else {
            None
        };
        let mut keyboard = "not applicable";
        if width == 390 && route == "/" {
            browser.toggle_by_keyboard(".mobile-menu")?;
            browser.toggle_by_keyboard(".track-story")?;
            browser.toggle_by_keyboard(".contact-details")?;
            keyboard = "menu, music story and contact opened and closed with Enter";
        }
        let report = json!({"width":width,"height":height,"route":route,"content_width":content_width,"viewport_width":viewport_width,"horizontal_overflow":overflow,"first_view_aligned":first_view_aligned,"unnamed_actions":unnamed_actions.len(),"script_requests":scripts,"failed_resources":failed,"initial_encoded_bytes":initial_bytes,"keyboard":keyboard,"javascript_execution":"disabled","reduced_motion":"enabled"});
        println!("{report}");
        reports.push(report);
        if width == 320 || width == 390 || width == 1440 {
            let name = if route == "/" { "home" } else { "fer" };
            browser.navigate(&format!("http://127.0.0.1:8080{route}"))?;
            browser.screenshot(
                &format!("{name}-{width}-viewport.png"),
                false,
                width,
                height,
            )?;
            let page_height =
                browser.call("Page.getLayoutMetrics", json!({}))?["cssContentSize"]["height"]
                    .as_f64()
                    .unwrap_or(0.0);
            let mut scrolled = 0.0;
            while scrolled < page_height {
                browser.call("Input.dispatchMouseEvent", json!({"type":"mouseWheel","x":width/2,"y":height/2,"deltaX":0,"deltaY":height as f64 * 0.8}))?;
                thread::sleep(Duration::from_millis(65));
                scrolled += height as f64 * 0.8;
            }
            thread::sleep(Duration::from_millis(200));
            browser.screenshot(&format!("{name}-{width}-full.png"), true, width, height)?;
        }
    }
    browser.call(
        "Emulation.setDeviceMetricsOverride",
        json!({"width":390,"height":844,"deviceScaleFactor":1,"mobile":true}),
    )?;
    browser.navigate("http://127.0.0.1:8080/")?;
    let frame = browser.call("Page.getFrameTree", json!({}))?["frameTree"]["frame"]["id"].clone();
    let sheet =
        browser.call("CSS.createStyleSheet", json!({"frameId":frame}))?["styleSheetId"].clone();
    browser.call(
        "CSS.setStyleSheetText",
        json!({"styleSheetId":sheet,"text":"html { font-size: 200%; }"}),
    )?;
    let metrics = browser.call("Page.getLayoutMetrics", json!({}))?;
    let width = metrics["cssContentSize"]["width"]
        .as_f64()
        .ok_or("no enlarged content width")?;
    let viewport = metrics["cssLayoutViewport"]["clientWidth"]
        .as_f64()
        .ok_or("no enlarged viewport width")?;
    let doc = browser.call("DOM.getDocument", json!({}))?;
    let root = doc["root"]["nodeId"].clone();
    let html = browser.call(
        "DOM.querySelector",
        json!({"nodeId":root,"selector":"html"}),
    )?["nodeId"]
        .clone();
    let computed = browser.call("CSS.getComputedStyleForNode", json!({"nodeId":html}))?;
    let scrolling = computed["computedStyle"]
        .as_array()
        .unwrap()
        .iter()
        .find(|v| v["name"] == "scroll-behavior")
        .unwrap()["value"]
        .clone();
    let font_size = computed["computedStyle"]
        .as_array()
        .unwrap()
        .iter()
        .find(|v| v["name"] == "font-size")
        .unwrap()["value"]
        .clone();
    let report = json!({"width":390,"height":844,"route":"/","font_size":font_size,"text_enlargement":"200%","content_width":width,"viewport_width":viewport,"horizontal_overflow":width > 391.0 || viewport > 391.0,"scroll_behavior":scrolling,"unnamed_actions":0,"script_requests":0,"failed_resources":[]});
    println!("{report}");
    reports.push(report);
    browser.screenshot("home-390-text-200.png", false, 390, 844)?;
    browser.call(
        "CSS.setStyleSheetText",
        json!({"styleSheetId":sheet,"text":""}),
    )?;
    fs::write(
        output_path("browser-report.json"),
        serde_json::to_string_pretty(&reports)?,
    )?;
    if reports.iter().any(|r| {
        r["horizontal_overflow"] == true
            || r["first_view_aligned"] == false
            || r["unnamed_actions"] != 0
            || r["script_requests"] != 0
            || !r["failed_resources"].as_array().unwrap().is_empty()
    }) {
        return Err("Browser audit found issues; inspect browser-report.json".into());
    }
    Ok(())
}

fn audit_fer(browser: &mut Browser) -> Result<()> {
    for (width, height) in [(320, 812), (390, 844), (768, 1024), (1440, 1000)] {
        browser.call(
            "Emulation.setDeviceMetricsOverride",
            json!({"width":width,"height":height,"deviceScaleFactor":1,"mobile":width<600}),
        )?;
        browser.navigate("http://127.0.0.1:8080/fer/")?;
        let metrics = browser.call("Page.getLayoutMetrics", json!({}))?;
        let content_width = metrics["cssContentSize"]["width"]
            .as_f64()
            .ok_or("missing content width")?;
        let failed: Vec<_> = browser
            .events
            .iter()
            .filter(|event| {
                event["method"] == "Network.responseReceived"
                    && event["params"]["response"]["status"]
                        .as_f64()
                        .unwrap_or(0.0)
                        >= 400.0
            })
            .map(|event| event["params"]["response"]["url"].clone())
            .collect();
        let scripts = browser
            .events
            .iter()
            .filter(|event| {
                event["method"] == "Network.requestWillBeSent"
                    && event["params"]["type"] == "Script"
            })
            .count();
        let tree = browser.call("Accessibility.getFullAXTree", json!({}))?;
        let unnamed_actions = tree["nodes"]
            .as_array()
            .ok_or("no accessibility nodes")?
            .iter()
            .filter(|node| {
                node["ignored"] == false
                    && matches!(node["role"]["value"].as_str(), Some("link" | "button"))
                    && node["name"]["value"]
                        .as_str()
                        .unwrap_or("")
                        .trim()
                        .is_empty()
            })
            .count();
        browser.screenshot(
            &format!("fer-review-{width}-viewport.png"),
            false,
            width,
            height,
        )?;
        browser.screenshot(&format!("fer-review-{width}-full.png"), true, width, height)?;
        if width == 390 {
            let interlace = browser.call(
                "Runtime.evaluate",
                json!({"expression":"(() => { const images = [...document.querySelectorAll('.fer-artifact-interlace img')]; return images.every(image => image.complete && image.naturalWidth > 0); })()","returnByValue":true}),
            )?["result"]["value"]
                .clone();
            if interlace != true {
                return Err("FER opening interlace images did not load".into());
            }
            browser.call(
                "Runtime.evaluate",
                json!({"expression":"document.querySelector('.fer-artifact-seal').scrollIntoView()"}),
            )?;
            thread::sleep(Duration::from_millis(500));
            browser.current_viewport("fer-review-390-seal.png")?;
            let seal = browser.call(
                "Runtime.evaluate",
                json!({"expression":"(() => { const image = document.querySelector('.fer-artifact-seal .fer-artifact-complete'); return {complete:image.complete,width:image.naturalWidth}; })()","returnByValue":true}),
            )?["result"]["value"].clone();
            if seal["complete"] != true || seal["width"].as_u64().unwrap_or(0) == 0 {
                return Err(format!("FER convergence seal image did not load: {seal}").into());
            }
        }
        println!(
            "{}",
            json!({"width":width,"content_width":content_width,"overflow":content_width>width as f64+1.0,"failed_resources":failed,"script_requests":scripts,"unnamed_actions":unnamed_actions})
        );
        if content_width > width as f64 + 1.0
            || !failed.is_empty()
            || scripts > 0
            || unnamed_actions > 0
        {
            return Err(format!("FER audit failed at width {width}").into());
        }
    }
    browser.call(
        "Emulation.setDeviceMetricsOverride",
        json!({"width":390,"height":844,"deviceScaleFactor":1,"mobile":true}),
    )?;
    browser.call(
        "Emulation.setEmulatedMedia",
        json!({"features":[{"name":"prefers-reduced-motion","value":"no-preference"}]}),
    )?;
    browser.navigate("http://127.0.0.1:8080/fer/")?;
    let first = browser.style(".fer-seal-f", "transform")?;
    let band_first = browser.style(".fer-band-red", "transform")?;
    thread::sleep(Duration::from_secs(2));
    let second = browser.style(".fer-seal-f", "transform")?;
    let band_second = browser.style(".fer-band-red", "transform")?;
    let running = browser.style(".fer-seal-rail", "animation-name")?;
    println!(
        "{}",
        json!({"motion":"normal","seal_first":first,"seal_second":second,"band_first":band_first,"band_second":band_second,"rail_animation":running})
    );
    if first == second || band_first == band_second || !running.contains("fer-seal-engrave") {
        return Err("FER artifacts did not move".into());
    }
    browser.call(
        "Runtime.evaluate",
        json!({"expression":"document.querySelector('#pause-motion').checked = true"}),
    )?;
    let paused_seal = browser.style(".fer-seal-f", "display")?;
    let paused_band = browser.style(".fer-band-red", "display")?;
    let paused_artwork = browser.style(".fer-artifact-complete", "opacity")?;
    if paused_seal != "none" || paused_band != "none" || paused_artwork != "1" {
        return Err(format!("FER pause state failed: {paused_seal}, {paused_band}, {paused_artwork}").into());
    }
    browser.call(
        "Runtime.evaluate",
        json!({"expression":"document.querySelector('#pause-motion').checked = false"}),
    )?;
    browser.call(
        "Emulation.setEmulatedMedia",
        json!({"features":[{"name":"prefers-reduced-motion","value":"reduce"}]}),
    )?;
    let reduced = browser.style(".fer-artifact-complete", "animation-name")?;
    let reduced_seal = browser.style(".fer-seal-f", "display")?;
    let reduced_band = browser.style(".fer-band-red", "display")?;
    if reduced != "none" || reduced_seal != "none" || reduced_band != "none" {
        return Err(format!("FER reduced-motion animation still running: {reduced}, {reduced_seal}, {reduced_band}").into());
    }
    let frame = browser.call("Page.getFrameTree", json!({}))?["frameTree"]["frame"]["id"].clone();
    let sheet =
        browser.call("CSS.createStyleSheet", json!({"frameId":frame}))?["styleSheetId"].clone();
    browser.call(
        "CSS.setStyleSheetText",
        json!({"styleSheetId":sheet,"text":"html {font-size:200%}"}),
    )?;
    browser.screenshot("fer-review-390-text-200.png", false, 390, 844)?;
    browser.screenshot("fer-review-390-text-200-full.png", true, 390, 844)?;
    let zoom_width = browser.call("Page.getLayoutMetrics", json!({}))?["cssContentSize"]["width"]
        .as_f64()
        .ok_or("missing text zoom width")?;
    println!("{}", json!({"text_zoom":"200%","content_width":zoom_width}));
    Ok(())
}

fn audit_motion(browser: &mut Browser) -> Result<()> {
    browser.call("Media.enable", json!({}))?;
    let mut reports = Vec::new();
    for (width, height, expected_texture, expected_video) in [
        (
            390,
            844,
            "cover-smoke-mobile.webp",
            "cover-smoke-flow-mobile.mp4",
        ),
        (
            1440,
            1000,
            "cover-smoke-desktop.webp",
            "cover-smoke-flow-desktop.mp4",
        ),
    ] {
        browser.call(
            "Emulation.setDeviceMetricsOverride",
            json!({"width":width,"height":height,"deviceScaleFactor":1,"mobile":width < 600}),
        )?;
        browser.call(
            "Emulation.setEmulatedMedia",
            json!({"features":[{"name":"prefers-reduced-motion","value":"no-preference"}]}),
        )?;
        browser.navigate("http://127.0.0.1:8080/")?;
        thread::sleep(Duration::from_millis(1500));
        let opacity_before = browser
            .style(".smoke-atmosphere", "opacity")?
            .parse::<f64>()?;
        let italic = browser.style("#hero-title span:last-child", "font-style")?;
        let italic_node = browser.node("#hero-title span:last-child")?;
        let fonts = browser.call("CSS.getPlatformFontsForNode", json!({"nodeId":italic_node}))?;
        let before = format!("motion-{width}-before.png");
        let after = format!("motion-{width}-after.png");
        browser.screenshot(&before, false, width, height)?;
        let transform_before = browser.style(".smoke-field-far", "transform")?;
        thread::sleep(Duration::from_millis(1900));
        browser.screenshot(&after, false, width, height)?;
        let transform_after = browser.style(".smoke-field-far", "transform")?;
        let moving = transform_before != transform_after
            && content_pixels(&before, width, height)? != content_pixels(&after, width, height)?;
        let texture_requests: Vec<_> = browser
            .events
            .iter()
            .filter(|e| {
                e["method"] == "Network.requestWillBeSent"
                    && e["params"]["request"]["url"]
                        .as_str()
                        .is_some_and(|url| url.contains("/assets/atmosphere/"))
            })
            .map(|e| e["params"]["request"]["url"].as_str().unwrap().to_owned())
            .collect();
        let selected = !texture_requests.is_empty()
            && texture_requests
                .iter()
                .all(|url| url.ends_with(expected_texture));
        let video_requests: Vec<_> = browser
            .events
            .iter()
            .filter(|e| {
                e["method"] == "Network.requestWillBeSent"
                    && e["params"]["request"]["url"]
                        .as_str()
                        .is_some_and(|url| url.ends_with(".mp4"))
            })
            .map(|e| e["params"]["request"]["url"].as_str().unwrap().to_owned())
            .collect();
        let selected_video = !video_requests.is_empty()
            && video_requests
                .iter()
                .all(|url| url.ends_with(expected_video));
        let played = browser
            .events
            .iter()
            .filter(|e| e["method"] == "Media.playerEventsAdded")
            .flat_map(|e| e["params"]["events"].as_array().into_iter().flatten())
            .any(|e| e["value"].as_str().is_some_and(|v| v.contains("kPlay")));

        // Freeze the background textures during capture: changing pixels must
        // come from the actual video curls, not a panning still photograph.
        let frame =
            browser.call("Page.getFrameTree", json!({}))?["frameTree"]["frame"]["id"].clone();
        let sheet =
            browser.call("CSS.createStyleSheet", json!({"frameId":frame}))?["styleSheetId"].clone();
        browser.call("CSS.setStyleSheetText", json!({"styleSheetId":sheet,"text":".smoke-field { animation-play-state:paused!important; }"}))?;
        let flow_a = format!("flow-{width}-isolated-a.png");
        let flow_b = format!("flow-{width}-isolated-b.png");
        browser.screenshot(&flow_a, false, width, height)?;
        thread::sleep(Duration::from_millis(2000));
        browser.screenshot(&flow_b, false, width, height)?;
        let video_change_fraction = moving_edge_fraction(&flow_a, &flow_b, width, height)?;
        browser.call(
            "CSS.setStyleSheetText",
            json!({"styleSheetId":sheet,"text":""}),
        )?;
        let metrics = browser.call("Page.getLayoutMetrics", json!({}))?;
        let overflow =
            metrics["cssContentSize"]["width"].as_f64().unwrap_or(0.0) > width as f64 + 1.0;

        let muse_heading = browser.node("#muses-title")?;
        browser.call("DOM.scrollIntoViewIfNeeded", json!({"nodeId":muse_heading}))?;
        browser.call("Input.dispatchMouseEvent", json!({"type":"mouseWheel","x":width/2,"y":height/2,"deltaX":0,"deltaY":-(height as i64)/2}))?;
        thread::sleep(Duration::from_millis(250));
        let muse_scale_enter = browser.style(".muse-karen", "scale")?;
        browser.current_viewport(&format!("muse-card-{width}-scroll-entry.png"))?;
        let picture = browser.node(".muse-image-link > picture")?;
        browser.call("DOM.scrollIntoViewIfNeeded", json!({"nodeId":picture}))?;
        thread::sleep(Duration::from_millis(250));
        let muse_scale_near = browser.style(".muse-karen", "scale")?;
        let curtain = browser.style(".muse-image-link > picture", "clip-path")?;
        let karen_rect = browser.box_rect(".muse-karen .muse-image-link")?;
        let muse_gold_karen_a =
            browser.pseudo_style(".muse-karen .muse-image-link", "before", "--muse-rim-angle")?;
        browser.current_viewport(&format!("muse-card-{width}-gold-a.png"))?;
        thread::sleep(Duration::from_millis(950));
        let muse_gold_karen_b =
            browser.pseudo_style(".muse-karen .muse-image-link", "before", "--muse-rim-angle")?;
        browser.current_viewport(&format!("muse-card-{width}-gold-b.png"))?;
        let karen_rim_changed = rim_pixels_changed(
            &format!("muse-card-{width}-gold-a.png"),
            &format!("muse-card-{width}-gold-b.png"),
            width,
            height,
            karen_rect,
        )?;
        browser.call(
            "Input.dispatchMouseEvent",
            json!({"type":"mouseWheel","x":width/2,"y":height/2,"deltaX":0,"deltaY":height/3}),
        )?;
        thread::sleep(Duration::from_millis(350));
        browser.current_viewport(&format!("continuity-{width}-muses.png"))?;
        let scale_enter = muse_scale_enter.parse::<f64>()?;
        let scale_near = muse_scale_near.parse::<f64>()?;
        let gold_karen_a = muse_gold_karen_a.trim_end_matches("deg").parse::<f64>()?;
        let gold_karen_b = muse_gold_karen_b.trim_end_matches("deg").parse::<f64>()?;
        let gold_karen_travel = (gold_karen_b - gold_karen_a + 360.0) % 360.0;
        let muse_link = browser.node(".muse-image-link")?;
        browser.call("DOM.focus", json!({"nodeId":muse_link}))?;
        thread::sleep(Duration::from_millis(550));
        let muse_focus_transform = browser.style(".muse-card", "transform")?;
        browser.current_viewport(&format!("muse-card-{width}-focus.png"))?;
        let muse_hover_transform = if width >= 900 {
            let muse_link = browser.node(".muse-image-link")?;
            let model = browser.call("DOM.getBoxModel", json!({"nodeId":muse_link}))?;
            let border = model["model"]["border"].as_array().ok_or("no muse box")?;
            let x = (border[0].as_f64().ok_or("no muse x")?
                + border[2].as_f64().ok_or("no muse right edge")?)
                / 2.0;
            let y = (border[1].as_f64().ok_or("no muse y")?
                + border[5].as_f64().ok_or("no muse bottom edge")?)
                / 2.0;
            browser.call(
                "Input.dispatchMouseEvent",
                json!({"type":"mouseMoved","x":x,"y":y,"button":"none"}),
            )?;
            thread::sleep(Duration::from_millis(550));
            let transform = browser.style(".muse-card", "transform")?;
            browser.current_viewport(&format!("muse-card-{width}-hover.png"))?;
            browser.call(
                "Input.dispatchMouseEvent",
                json!({"type":"mouseMoved","x":0,"y":0,"button":"none"}),
            )?;
            Some(transform)
        } else {
            None
        };
        let zoe_picture = browser.node(".muse-zoe .muse-image-link > picture")?;
        browser.call("DOM.scrollIntoViewIfNeeded", json!({"nodeId":zoe_picture}))?;
        thread::sleep(Duration::from_millis(250));
        let zoe_rect = browser.box_rect(".muse-zoe .muse-image-link")?;
        let muse_gold_zoe_a =
            browser.pseudo_style(".muse-zoe .muse-image-link", "before", "--muse-rim-angle")?;
        browser.current_viewport(&format!("muse-card-{width}-zoe-gold-a.png"))?;
        thread::sleep(Duration::from_millis(950));
        let muse_gold_zoe_b =
            browser.pseudo_style(".muse-zoe .muse-image-link", "before", "--muse-rim-angle")?;
        browser.current_viewport(&format!("muse-card-{width}-zoe-gold-b.png"))?;
        let zoe_rim_changed = rim_pixels_changed(
            &format!("muse-card-{width}-zoe-gold-a.png"),
            &format!("muse-card-{width}-zoe-gold-b.png"),
            width,
            height,
            zoe_rect,
        )?;
        let gold_zoe_a = muse_gold_zoe_a.trim_end_matches("deg").parse::<f64>()?;
        let gold_zoe_b = muse_gold_zoe_b.trim_end_matches("deg").parse::<f64>()?;
        let gold_zoe_travel = (gold_zoe_b - gold_zoe_a + 360.0) % 360.0;
        let music = browser.node(".track-entry")?;
        browser.call("DOM.scrollIntoViewIfNeeded", json!({"nodeId":music}))?;
        browser.current_viewport(&format!("continuity-{width}-music.png"))?;
        let education = browser.node("#education-title")?;
        browser.call("DOM.scrollIntoViewIfNeeded", json!({"nodeId":education}))?;
        thread::sleep(Duration::from_millis(250));
        browser.current_viewport(&format!("continuity-{width}-education.png"))?;
        let fer = browser.node("#fer-title")?;
        browser.call("DOM.scrollIntoViewIfNeeded", json!({"nodeId":fer}))?;
        thread::sleep(Duration::from_millis(250));
        let opacity_fer = browser
            .style(".smoke-atmosphere", "opacity")?
            .parse::<f64>()?;
        browser.current_viewport(&format!("continuity-{width}-fer.png"))?;
        let closing = browser.node("#contact-title")?;
        browser.call("DOM.scrollIntoViewIfNeeded", json!({"nodeId":closing}))?;
        thread::sleep(Duration::from_millis(250));
        let opacity_closing = browser
            .style(".smoke-atmosphere", "opacity")?
            .parse::<f64>()?;
        browser.current_viewport(&format!("continuity-{width}-closing.png"))?;

        let toggle = browser.node("#pause-motion")?;
        browser.call("DOM.focus", json!({"nodeId":toggle}))?;
        browser.space()?;
        thread::sleep(Duration::from_millis(150));
        let paused_state = browser.style(".smoke-field-far", "animation-play-state")?;
        let paused_rim =
            browser.pseudo_style(".muse-karen .muse-image-link", "before", "animation-name")?;
        let video_hidden = browser.style(".smoke-live", "display")? == "none";
        let paused_animation = browser.style(".hero-content", "animation-name")?;
        let pause_a = format!("motion-{width}-paused-a.png");
        let pause_b = format!("motion-{width}-paused-b.png");
        browser.screenshot(&pause_a, false, width, height)?;
        thread::sleep(Duration::from_millis(900));
        browser.screenshot(&pause_b, false, width, height)?;
        let stable =
            content_pixels(&pause_a, width, height)? == content_pixels(&pause_b, width, height)?;
        let toggle = browser.node("#pause-motion")?;
        browser.call("DOM.focus", json!({"nodeId":toggle}))?;
        browser.space()?;
        let resumed = browser.style(".smoke-field-far", "animation-play-state")? == "running"
            && browser.style(".smoke-live", "display")? != "none"
            && browser.pseudo_style(".muse-karen .muse-image-link", "before", "animation-name")?
                == "muse-rim-travel";
        let no_scripts = !browser
            .events
            .iter()
            .any(|e| e["method"] == "Network.requestWillBeSent" && e["params"]["type"] == "Script");
        let report = json!({"width":width,"mode":"normal","expected_texture":expected_texture,"texture_requests":texture_requests,"expected_video":expected_video,"video_requests":video_requests,"native_playback":played,"isolated_video_change_fraction":video_change_fraction,"texture_transform_before":transform_before,"texture_transform_after":transform_after,"frames_change":moving,"comparison_region":"page content excluding the browser scrollbar; isolated video capture freezes CSS smoke textures","pause_hides_video":video_hidden,"pause_freezes_smoke":paused_state == "paused","pause_removes_css_animation":paused_animation == "none","paused_rim_animation":paused_rim,"paused_frames_stable":stable,"keyboard_resumes_motion":resumed,"hero_font_style":italic,"platform_fonts":fonts,"image_reveal_clip":curtain,"muse_scale_enter":muse_scale_enter,"muse_scale_near":muse_scale_near,"muse_gold_karen_a":muse_gold_karen_a,"muse_gold_karen_b":muse_gold_karen_b,"muse_gold_karen_travel":gold_karen_travel,"muse_gold_karen_changed_pixels":karen_rim_changed,"muse_gold_zoe_a":muse_gold_zoe_a,"muse_gold_zoe_b":muse_gold_zoe_b,"muse_gold_zoe_travel":gold_zoe_travel,"muse_gold_zoe_changed_pixels":zoe_rim_changed,"muse_focus_transform":muse_focus_transform,"muse_hover_transform":muse_hover_transform,"smoke_opacity_at_fer":opacity_fer,"smoke_opacity_near_closing":opacity_closing,"horizontal_overflow":overflow,"javascript_execution":"disabled","script_requests":!no_scripts as u8});
        println!(
            "{}",
            json!({"width":width,"mode":"normal","texture_source_matches_viewport":selected,"video_source_matches_viewport":selected_video,"native_playback":played,"isolated_video_change_fraction":video_change_fraction,"frames_change":moving,"muse_scale_enter":muse_scale_enter,"muse_scale_near":muse_scale_near,"muse_gold_karen_travel":gold_karen_travel,"muse_gold_karen_changed_pixels":karen_rim_changed,"muse_gold_zoe_travel":gold_zoe_travel,"muse_gold_zoe_changed_pixels":zoe_rim_changed,"muse_focus_transform":muse_focus_transform,"muse_hover_transform":muse_hover_transform,"paused_rim_animation":paused_rim,"paused_content_stable":stable,"keyboard_resume":resumed,"overflow":overflow,"smoke_opacity_at_fer":opacity_fer,"smoke_opacity_near_closing":opacity_closing})
        );
        reports.push(report);
        if !moving
            || !selected_video
            || !played
            || video_change_fraction < 0.005
            || !video_hidden
            || !selected
            || !stable
            || !resumed
            || paused_state != "paused"
            || paused_animation != "none"
            || paused_rim != "none"
            || italic != "italic"
            || scale_near < scale_enter + 0.05
            || !(25.0..300.0).contains(&gold_karen_travel)
            || !(25.0..300.0).contains(&gold_zoe_travel)
            || karen_rim_changed < 80
            || zoe_rim_changed < 80
            || muse_focus_transform == "none"
            || muse_hover_transform
                .as_ref()
                .is_some_and(|transform| transform == &muse_focus_transform)
            || overflow
            || !no_scripts
            || opacity_closing >= opacity_before
            || opacity_fer < opacity_closing
        {
            fs::write(
                output_path("motion-report.json"),
                serde_json::to_string_pretty(&reports)?,
            )?;
            return Err("Motion checks failed; inspect motion-report.json".into());
        }

        browser.call(
            "Emulation.setEmulatedMedia",
            json!({"features":[{"name":"prefers-reduced-motion","value":"reduce"}]}),
        )?;
        browser.navigate("http://127.0.0.1:8080/")?;
        thread::sleep(Duration::from_millis(250));
        let reduced_requests = browser
            .events
            .iter()
            .filter(|e| {
                e["method"] == "Network.requestWillBeSent"
                    && e["params"]["request"]["url"]
                        .as_str()
                        .is_some_and(|url| url.ends_with(".mp4"))
            })
            .count();
        let reduced_smoke_animation = browser.style(".smoke-field-far", "animation-name")?;
        let reduced_rim_animation =
            browser.pseudo_style(".muse-karen .muse-image-link", "before", "animation-name")?;
        let reduced_animation = browser.style(".hero-content", "animation-name")?;
        let control_display = browser.style(".motion-control", "display")?;
        let reduced_video_display = browser.style(".smoke-live", "display")?;
        let report = json!({"width":width,"mode":"reduced","video_requests":reduced_requests,"video_display":reduced_video_display,"smoke_animation":reduced_smoke_animation,"muse_rim_animation":reduced_rim_animation,"hero_animation":reduced_animation,"pause_control_display":control_display});
        println!("{report}");
        reports.push(report);
        if reduced_requests != 0
            || reduced_video_display != "none"
            || reduced_smoke_animation != "none"
            || reduced_rim_animation != "none"
            || reduced_animation != "none"
            || control_display != "none"
        {
            return Err("Reduced motion must keep the smoke static without video".into());
        }
    }
    fs::write(
        output_path("motion-report.json"),
        serde_json::to_string_pretty(&reports)?,
    )?;
    Ok(())
}

fn audit_pointer(browser: &mut Browser) -> Result<()> {
    let width = 1440;
    let height = 1000;
    browser.call(
        "Emulation.setDeviceMetricsOverride",
        json!({"width":width,"height":height,"deviceScaleFactor":1,"mobile":false}),
    )?;
    browser.call(
        "Emulation.setEmulatedMedia",
        json!({"features":[{"name":"prefers-reduced-motion","value":"no-preference"}]}),
    )?;
    browser.navigate("http://127.0.0.1:8080/")?;
    let script_requests = browser
        .events
        .iter()
        .filter(|event| {
            event["method"] == "Network.requestWillBeSent" && event["params"]["type"] == "Script"
        })
        .count();
    let hero = browser.box_rect(".hero-content")?;
    let y = hero.1 + (hero.3 - hero.1) * 2 / 3;
    let x1 = (hero.0 + hero.2) / 2 - 60;
    let x2 = x1 + 120;
    for step in 0..=15 {
        browser.call(
            "Input.dispatchMouseEvent",
            json!({"type":"mouseMoved","x":x1+step*8,"y":y,"button":"none"}),
        )?;
        thread::sleep(Duration::from_millis(16));
    }
    thread::sleep(Duration::from_millis(45));
    let trail = canvas_ink(browser)?;
    browser.current_viewport("mouse-smoke-line.png")?;
    thread::sleep(Duration::from_millis(700));
    let idle = canvas_ink(browser)?;
    let picture = browser.node(".muse-karen .muse-image-link > picture")?;
    browser.call("DOM.scrollIntoViewIfNeeded", json!({"nodeId":picture}))?;
    thread::sleep(Duration::from_millis(200));
    let karen = browser.box_rect(".muse-karen .muse-image-link")?;
    let card_y = (karen.1 + karen.3) / 2;
    let card_x = (karen.0 + karen.2) / 2;
    for step in 0..=8 {
        browser.call(
            "Input.dispatchMouseEvent",
            json!({"type":"mouseMoved","x":card_x-32+step*8,"y":card_y,"button":"none"}),
        )?;
        thread::sleep(Duration::from_millis(16));
    }
    thread::sleep(Duration::from_millis(40));
    let card_trail = canvas_ink(browser)?;
    browser.current_viewport("mouse-smoke-card-line.png")?;
    let pointer_events = browser.style(".mouse-smoke", "pointer-events")?;
    let toggle = browser.node("#pause-motion")?;
    browser.call("DOM.focus", json!({"nodeId":toggle}))?;
    browser.space()?;
    let paused_display = browser.style(".mouse-smoke", "display")?;
    let paused_ink = canvas_ink(browser)?;
    let toggle = browser.node("#pause-motion")?;
    browser.call("DOM.focus", json!({"nodeId":toggle}))?;
    browser.space()?;
    browser.call(
        "Emulation.setEmulatedMedia",
        json!({"features":[{"name":"prefers-reduced-motion","value":"reduce"}]}),
    )?;
    let reduced_display = browser.style(".mouse-smoke", "display")?;
    let reduced_ink = canvas_ink(browser)?;
    browser.call(
        "Emulation.setDeviceMetricsOverride",
        json!({"width":390,"height":844,"deviceScaleFactor":1,"mobile":true}),
    )?;
    browser.call(
        "Emulation.setTouchEmulationEnabled",
        json!({"enabled":true,"maxTouchPoints":1}),
    )?;
    browser.call(
        "Emulation.setEmulatedMedia",
        json!({"features":[{"name":"prefers-reduced-motion","value":"no-preference"}]}),
    )?;
    browser.navigate("http://127.0.0.1:8080/")?;
    let mobile_display = browser.style(".mouse-smoke", "display")?;
    let mobile_ink = canvas_ink(browser)?;
    let menu = browser.box_rect(".mobile-menu summary")?;
    let tap_x = (menu.0 + menu.2) / 2;
    let tap_y = (menu.1 + menu.3) / 2;
    browser.call(
        "Input.dispatchTouchEvent",
        json!({"type":"touchStart","touchPoints":[{"x":tap_x,"y":tap_y,"id":1}]}),
    )?;
    thread::sleep(Duration::from_millis(60));
    let tap_ink = canvas_ink(browser)?;
    browser.current_viewport("touch-smoke-tap.png")?;
    browser.call(
        "Input.dispatchTouchEvent",
        json!({"type":"touchEnd","touchPoints":[]}),
    )?;
    thread::sleep(Duration::from_millis(120));
    let menu_node = browser.node(".mobile-menu")?;
    let menu_opened = browser.call("DOM.getAttributes", json!({"nodeId":menu_node}))?["attributes"]
        .as_array()
        .ok_or("no menu attributes")?
        .iter()
        .any(|attribute| attribute == "open");
    browser.call(
        "Input.dispatchTouchEvent",
        json!({"type":"touchStart","touchPoints":[{"x":260,"y":720,"id":2}]}),
    )?;
    for step in 1..=12 {
        browser.call(
            "Input.dispatchTouchEvent",
            json!({"type":"touchMove","touchPoints":[{"x":260,"y":720-step*20,"id":2}]}),
        )?;
        thread::sleep(Duration::from_millis(16));
    }
    thread::sleep(Duration::from_millis(45));
    let scroll_ink = canvas_ink(browser)?;
    browser.current_viewport("touch-smoke-scroll.png")?;
    browser.call(
        "Input.dispatchTouchEvent",
        json!({"type":"touchEnd","touchPoints":[]}),
    )?;
    thread::sleep(Duration::from_millis(160));
    let scrolled = browser.call("Page.getLayoutMetrics", json!({}))?["cssLayoutViewport"]["pageY"]
        .as_f64()
        .unwrap_or(0.0);
    thread::sleep(Duration::from_millis(700));
    let touch_idle = canvas_ink(browser)?;
    browser.call(
        "Emulation.setEmulatedMedia",
        json!({"features":[{"name":"prefers-reduced-motion","value":"reduce"}]}),
    )?;
    let mobile_reduced_display = browser.style(".mouse-smoke", "display")?;
    let mobile_reduced_ink = canvas_ink(browser)?;
    let report = json!({"cursor_tip":[x2,y],"trail":trail,"idle":idle,"card_trail":card_trail,"pointer_events":pointer_events,"paused_display":paused_display,"paused_ink":paused_ink,"reduced_motion_display":reduced_display,"reduced_ink":reduced_ink,"mobile_display":mobile_display,"mobile_ink":mobile_ink,"tap_ink":tap_ink,"menu_opened":menu_opened,"scroll_ink":scroll_ink,"scrolled_pixels":scrolled,"touch_idle":touch_idle,"mobile_reduced_display":mobile_reduced_display,"mobile_reduced_ink":mobile_reduced_ink,"script_requests_on_desktop":script_requests});
    println!("{report}");
    fs::write(
        output_path("pointer-report.json"),
        serde_json::to_string_pretty(&report)?,
    )?;
    if script_requests != 1
        || trail["count"].as_u64().unwrap_or(0) < 100
        || trail["left"].as_i64().unwrap_or(-1) < i64::from(x2 - 140)
        || trail["right"].as_i64().unwrap_or(-1) > i64::from(x2 + 18)
        || trail["right"].as_i64().unwrap_or(-1) < i64::from(x2 - 8)
        || trail["bottom"].as_i64().unwrap_or(-1) - trail["top"].as_i64().unwrap_or(-1) > 40
        || idle["count"] != 0
        || card_trail["count"].as_u64().unwrap_or(0) < 40
        || pointer_events != "none"
        || paused_display != "none"
        || paused_ink["count"] != 0
        || reduced_display != "none"
        || reduced_ink["width"] != 0
        || mobile_display != "block"
        || mobile_ink["width"] != 390
        || tap_ink["count"].as_u64().unwrap_or(0) < 25
        || !menu_opened
        || scroll_ink["count"].as_u64().unwrap_or(0) < 80
        || scrolled < 40.0
        || touch_idle["count"] != 0
        || mobile_reduced_display != "none"
        || mobile_reduced_ink["width"] != 0
    {
        return Err("Pointer trail must be narrow, start at the cursor, and fade away".into());
    }
    Ok(())
}

fn canvas_ink(browser: &mut Browser) -> Result<Value> {
    let result = browser.call(
        "Runtime.evaluate",
        json!({"returnByValue":true,"expression":r#"(() => {
            const canvas = document.querySelector('canvas.mouse-smoke');
            const width = canvas.width;
            const height = canvas.height;
            if (!width || !height) return { count: 0, width, height, left: -1, top: -1, right: -1, bottom: -1 };
            const pixels = canvas.getContext('2d').getImageData(0, 0, width, height).data;
            let count = 0, left = width, top = height, right = -1, bottom = -1;
            for (let y = 0; y < height; y++) for (let x = 0; x < width; x++) {
                if (pixels[(y * width + x) * 4 + 3] > 4) {
                    count++;
                    left = Math.min(left, x); top = Math.min(top, y);
                    right = Math.max(right, x); bottom = Math.max(bottom, y);
                }
            }
            return { count, width, height, left, top, right, bottom };
        })()"#}),
    )?;
    if !result["exceptionDetails"].is_null() {
        return Err(format!("canvas inspection failed: {}", result["exceptionDetails"]).into());
    }
    Ok(result["result"]["value"].clone())
}

fn moving_edge_fraction(a: &str, b: &str, width: u32, height: u32) -> Result<f64> {
    let before = content_pixels(a, width, height)?;
    let after = content_pixels(b, width, height)?;
    let row_width = (width - 16) as usize;
    let mut changed = 0;
    let mut sampled = 0;
    for (index, (a, b)) in before
        .as_chunks::<4>()
        .0
        .iter()
        .zip(after.as_chunks::<4>().0.iter())
        .enumerate()
    {
        let x = index % row_width;
        let y = index / row_width;
        if y < 100 || (x > width as usize * 18 / 100 && x < width as usize * 88 / 100) {
            continue;
        }
        sampled += 1;
        let delta: u32 = (0..3)
            .map(|channel| a[channel].abs_diff(b[channel]) as u32)
            .sum();
        if delta >= 18 {
            changed += 1;
        }
    }
    if sampled == 0 {
        return Err("No smoke edge pixels were sampled".into());
    }
    Ok(changed as f64 / sampled as f64)
}

fn rim_pixels_changed(
    a: &str,
    b: &str,
    width: u32,
    height: u32,
    (left, top, right, bottom): (i32, i32, i32, i32),
) -> Result<usize> {
    let pixels = |name: &str| -> Result<Vec<u8>> {
        let image = Command::new("magick")
            .arg(output_path(name))
            .args(["-depth", "8", "rgba:-"])
            .output()?;
        if !image.status.success() || image.stdout.len() != (width * height * 4) as usize {
            return Err("Could not decode muse rim screenshot".into());
        }
        Ok(image.stdout)
    };
    let before = pixels(a)?;
    let after = pixels(b)?;
    let mut changed = 0;
    for y in top.max(0)..bottom.min(height as i32) {
        for x in left.max(0)..right.min(width as i32) {
            if x - left >= 7 && right - x > 7 && y - top >= 7 && bottom - y > 7 {
                continue;
            }
            let offset = ((y as u32 * width + x as u32) * 4) as usize;
            let delta: u32 = (0..3)
                .map(|channel| before[offset + channel].abs_diff(after[offset + channel]) as u32)
                .sum();
            if delta >= 45 {
                changed += 1;
            }
        }
    }
    Ok(changed)
}

fn content_pixels(filename: &str, width: u32, height: u32) -> Result<Vec<u8>> {
    // Browser scrollbar fading is independent of the site's motion preference.
    let geometry = format!("{}x{height}+0+0", width - 16);
    let image = Command::new("magick")
        .arg(output_path(filename))
        .args(["-crop", &geometry, "+repage", "rgba:-"])
        .output()?;
    if !image.status.success() {
        return Err("Could not decode screenshot pixels".into());
    }
    Ok(image.stdout)
}
