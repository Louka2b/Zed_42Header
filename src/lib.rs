use zed_extension_api as zed;

struct FortyTwoHeader;

impl zed::Extension for FortyTwoHeader {
    fn new() -> Self {
        Self
    }

    // Note: As of zed_extension_api 0.7.0, there is no native `on_save` hook in the Extension trait.
    // The code below is what was intended by the author. To make this work,
    // a Language Server (LSP) or a Task-based approach is currently required.
    
    // fn on_save(&mut self, buffer: &mut zed::Buffer) -> zed::Result<()> {
    //     let username = zed::settings_get("42header.username").unwrap_or("user".to_string());
    //     let email = zed::settings_get("42header.email").unwrap_or("user@student.42.fr".to_string());
    //     let text = buffer.text();
    //     let timestamp = "2026/06/13 12:11:58"; 
    //     if text.contains("/* ************************************************************************** */") {
    //         let new_text = text.replace(
    //             |line: &str| line.starts_with("/* Updated:"),
    //             &format!("/* Updated: {} by {}             ###   ########.fr       */", timestamp, username)
    //         );
    //         if new_text != text {
    //             buffer.edit(0..text.len(), &new_text);
    //             zed::show_message("42 Header updated!");
    //         }
    //     } else {
    //         let header = format!(
    // "/* ************************************************************************** */
    // /*                                                                            */
    // /*                                                        :::      ::::::::   */
    // /*   file                                               :+:      :+:    :+:   */
    // /*                                                    +:+ +:+         +:+     */
    // /*   By: {} <{}>                                  +#+  +:+       +#+        */
    // /*                                                +#+#+#+#+#+   +#+           */
    // /*   Created: 2026/06/13 12:11:58 by {}              #+#    #+#             */
    // /*   Updated: 2026/06/13 12:11:58 by {}             ###   ########.fr       */
    // /*                                                                            */
    // /* ************************************************************************** */
    // 
    // ", username, email, username, username);
    //         
    //         buffer.insert(0, &header);
    //         zed::show_message("42 Header inserted!");
    //     }
    //     Ok(())
    // }
}

// Example of how you might implement a slash command if you wanted to trigger it via the Assistant
// impl zed::SlashCommand for FortyTwoHeader {
//     fn run(
//         &self,
//         _command: zed::SlashCommand,
//         _args: Vec<String>,
//         _worktree: &zed::Worktree,
//     ) -> zed::Result<zed::SlashCommandOutput, String> {
//         Ok(zed::SlashCommandOutput {
//             text: "42 Header logic is ready but on_save is not yet supported by Zed WASM API.".to_string(),
//             sections: vec![],
//         })
//     }
// }

zed::register_extension!(FortyTwoHeader);
