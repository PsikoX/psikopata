use dioxus::prelude::*;

/// An original FER seal: engraved circles, three distinct vertices and three
/// luminous routes towards the core. It contains no borrowed religious marks.
#[component]
pub fn FerSigil() -> Element {
    rsx! {
        svg {
            class: "fer-seal",
            view_box: "0 0 600 600",
            fill: "none",
            "aria-hidden": "true",
            circle { class: "fer-seal-plate-glow", cx: "300", cy: "300", r: "257" }
            g { class: "fer-seal-dial",
                circle { class: "fer-seal-rim-outer", cx: "300", cy: "300", r: "255" }
                circle { class: "fer-seal-rim-ticks", cx: "300", cy: "300", r: "247" }
                circle { class: "fer-seal-rim-inner", cx: "300", cy: "300", r: "237" }
                circle { class: "fer-seal-rim-dashes", cx: "300", cy: "300", r: "226" }
            }
            g { class: "fer-seal-compass",
                path { d: "M300 13V35 M300 565V587 M13 300H35 M565 300H587" }
                path { d: "M82 82L98 98 M502 502L518 518 M518 82L502 98 M98 502L82 518" }
                path { d: "M300 18L306 30L300 42L294 30Z M300 558L306 570L300 582L294 570Z" }
                path { d: "M18 300L30 294L42 300L30 306Z M558 300L570 294L582 300L570 306Z" }
            }
            path { class: "fer-seal-triangle-glow", d: "M300 52L108 474H492Z" }
            path { class: "fer-seal-triangle-gold", d: "M300 52L108 474H492Z" }
            path { class: "fer-seal-triangle-fine", d: "M300 67L123 464H477Z" }
            path { class: "fer-seal-triangle-inner", d: "M300 113L157 440H443Z" }
            path { class: "fer-seal-geometry", d: "M300 52V474 M108 474L396 264 M492 474L204 264 M157 440L300 312L443 440" }
            g { class: "fer-seal-conduits",
                path { class: "fer-seal-conduit fer-seal-conduit-f", d: "M300 86V312" }
                path { class: "fer-seal-conduit fer-seal-conduit-e", d: "M129 456L300 312" }
                path { class: "fer-seal-conduit fer-seal-conduit-r", d: "M471 456L300 312" }
            }
            g { class: "fer-seal-flourishes",
                path { d: "M172 310C204 287 225 294 244 309 M428 310C396 287 375 294 356 309" }
                path { d: "M194 325L210 309L194 293 M406 325L390 309L406 293" }
                path { d: "M300 189L311 211L300 233L289 211Z M211 396L222 418L211 440L200 418Z M389 396L400 418L389 440L378 418Z" }
            }
            g { class: "fer-seal-corners",
                circle { cx: "300", cy: "52", r: "32" }
                circle { cx: "108", cy: "474", r: "32" }
                circle { cx: "492", cy: "474", r: "32" }
                circle { cx: "300", cy: "52", r: "39" }
                circle { cx: "108", cy: "474", r: "39" }
                circle { cx: "492", cy: "474", r: "39" }
            }
            path { class: "fer-seal-core-geometry", d: "M300 268L344 312L300 356L256 312Z M300 281L331 312L300 343L269 312Z" }
            circle { class: "fer-seal-core-ring", cx: "300", cy: "312", r: "53" }
            circle { class: "fer-seal-core-dashes", cx: "300", cy: "312", r: "63" }
            g { class: "fer-seal-sparks",
                path { d: "M300 142L303 150L311 153L303 156L300 164L297 156L289 153L297 150Z" }
                path { d: "M149 357L152 365L160 368L152 371L149 379L146 371L138 368L146 365Z" }
                path { d: "M451 357L454 365L462 368L454 371L451 379L448 371L440 368L448 365Z" }
                path { d: "M79 302L82 310L90 313L82 316L79 324L76 316L68 313L76 310Z" }
                path { d: "M521 302L524 310L532 313L524 316L521 324L518 316L510 313L518 310Z" }
            }
        }
    }
}
