/*
 * Copyright (c) Dave Williams <dave@dave.io>.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::sync::OnceLock;

use askama::Template;
use serde::{Deserialize, Serialize};

use crate::{BunnypmslCommandInfo, BunnypmslCommandRegistry, BunnypmslConfig};

static LANDING_PAGE_HTML_CACHE: OnceLock<String> = OnceLock::new();

/// Data structure for a command binding (displayed in cards)
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct BindingData {
    pub command: String,
    pub description: String,
    pub example: String,
}

impl From<BunnypmslCommandInfo> for BindingData {
    fn from(info: BunnypmslCommandInfo) -> Self {
        Self {
            command: info
                .bindings
                .first()
                .unwrap_or(&"(default)".to_string())
                .clone(),
            description: info.description,
            example: info.example,
        }
    }
}

/// Askama template for the landing page
#[derive(Template)]
#[template(path = "landing.html")]
struct LandingPageTemplate {
    server_display_url: String,
    example_url: String,
    search_engine_url: String,
    bindings: Vec<BindingData>,
}

/// Render the landing page HTML with the given config
pub fn render_landing_page_html(config: &BunnypmslConfig) -> String {
    LANDING_PAGE_HTML_CACHE
        .get_or_init(|| {
            let display_url = config.server.get_display_url();

            // Collect and sort bindings alphabetically
            let mut bindings: Vec<BindingData> = BunnypmslCommandRegistry::get_all_commands()
                .iter()
                .map(|cmd| (*cmd).clone().into())
                .collect();
            bindings.sort_by(|a, b| a.command.to_lowercase().cmp(&b.command.to_lowercase()));

            let template = LandingPageTemplate {
                server_display_url: display_url.clone(),
                example_url: format!("{}/?cmd=gh daveio/bunnypmsl", display_url),
                search_engine_url: format!("{}/?cmd=%s", display_url),
                bindings,
            };

            template.render().unwrap_or_else(|e| {
                format!(
                    "<html><body><h1>Template Error</h1><p>{}</p></body></html>",
                    e
                )
            })
        })
        .clone()
}
