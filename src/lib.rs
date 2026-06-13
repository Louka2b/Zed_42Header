use zed_extension_api::{self as zed, Result};

struct FortyTwoHeader;

impl zed::Extension for FortyTwoHeader {
    fn new() -> Self {
        Self
    }

    fn on_save(&mut self, buffer: &mut zed::Buffer) -> Result<()> {
        let username = zed::settings_get("42header.username").unwrap_or("user".to_string());
        let email = zed::settings_get("42header.email").unwrap_or("user@student.42.fr".to_string());
        let text = buffer.text();
        let timestamp = "2026/06/13 12:11:58"; 
        if text.contains("/* ************************************************************************** */") {
            let new_text = text.replace(
                |line: &str| line.starts_with("/* Updated:"),
                &format!("/* Updated: {} by {}             ###   ########.fr       */", timestamp, username)
            );
            if new_text != text {
                buffer.edit(0..text.len(), &new_text);
                zed::show_message("42 Header updated!");
            }
        } else {
            let header = format!(
"/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   file                                               :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: {} <{}>                                  +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2026/06/13 12:11:58 by {}              #+#    #+#             */
/*   Updated: 2026/06/13 12:11:58 by {}             ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

", username, email, username, username);
            
            buffer.insert(0, &header);
            zed::show_message("42 Header inserted!");
        }
        Ok(())
    }
}

zed::register_extension!(FortyTwoHeader);
