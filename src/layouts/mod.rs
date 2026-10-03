use crate::{
    components::{Brand, BrandName, ButtonLink, ExternalLink},
    content,
};
use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    rsx! {
        header { class: "site-header",
            div { class: "header-inner container",
                Brand {}
                nav { class: "desktop-nav", "aria-label": "Navegación principal",
                    for link in content::TOP_NAV { a { href: link.href, "{link.label}" } }
                }
                div { class: "header-end",
                    span { class: "locale", "ES / VE" }
                    a { class: "header-invitation", href: "/#las-7", "LAS 7" span { "aria-hidden": "true", " ✦" } }
                    details { class: "mobile-menu",
                        summary { "aria-label": "Abrir o cerrar la navegación", span { "MENÚ" } span { class: "menu-symbol", "aria-hidden": "true", "+" } }
                        nav { "aria-label": "Navegación móvil",
                            for link in content::TOP_NAV { a { href: link.href, "{link.label}" } }
                            a { href: "/#education", "EDUCACIÓN" }
                            a { href: "/#las-7", "LAS 7" }
                            a { href: "/fer/", "DESCUBRIR FER" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "site-footer",
            div { class: "container footer-top",
                div { class: "footer-identity", Brand { class: "footer-brand" } p { "{content::copy::FOOTER_DESCRIPTION}" } }
                nav { class: "footer-nav", "aria-label": "Explorar PSIKOPAPA",
                    for link in content::FOOTER_NAV { a { href: link.href, "{link.label}" } }
                }
            }
            div { class: "container footer-bottom",
                p { "© 2026 " BrandName {} span { " · " } "HECHO PARA ELLAS." }
                div { class: "social-links", "aria-label": "Plataformas musicales y sociales",
                    for (label, url) in content::SOCIAL_LINKS {
                        if let Some(href) = url { ExternalLink { href: *href, label: *label } }
                        else { span { class: "unavailable-link", title: "Enlace oficial todavía no disponible", "{label}" } }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Contact() -> Element {
    rsx! {
        section { class: "contact-section", id: "contact", "aria-labelledby": "contact-title",
            div { class: "container contact-inner",
                div { p { class: "eyebrow", "{content::copy::CONTACT_KICKER}" } h2 { id: "contact-title", "{content::copy::CONTACT_HEADING}" } }
                div { class: "contact-action",
                    if let Some(email) = content::CONTACT_EMAIL {
                        a { class: "button button-crimson", href: "mailto:{email}", "HABLEMOS" }
                    } else if let Some(href) = content::CONTACT_WHATSAPP {
                        a { class: "button button-crimson", href, target: "_blank", rel: "noopener noreferrer", "HABLEMOS" }
                    } else {
                        details { class: "contact-details",
                            summary { "CONTACTO" span { "aria-hidden": "true", "+" } }
                            p { "{content::copy::CONTACT_UNAVAILABLE}" }
                        }
                    }
                    p { "{content::copy::CONTACT_LOCATION}" }
                }
            }
        }
    }
}

#[component]
pub fn ClosingInvitation() -> Element {
    rsx! { div { class: "fer-return", ButtonLink { href: "/#las-7", label: "CONOCER LAS 7", secondary: true } } }
}
