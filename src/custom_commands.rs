use std::{thread, time};
use std::fs::File;
use std::io::BufReader;
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::ClientConfig;
use twitch_irc::SecureTCPTransport;
use twitch_irc::TwitchIRCClient;
use twitch_irc::message::{IRCMessage, ServerMessage};
use std::process::Command;
use winput::{Vk, Button};
use rodio::{Decoder, OutputStream, source::Source};



//mod recorder;
#[tokio::main]
pub async fn main() {
    // default configuration is to join chat as anonymous.
    let config = ClientConfig::default();
    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);


    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            match message{
                ServerMessage::Privmsg(msg) =>{
                    
                    for badge in msg.badges{
                        for word in msg.message_text.split(" ")
                        {

                            if badge.name != String::from("subscriber"){
                           
                                for word in msg.message_text.split(" "){
                                    thread::spawn(|| {custom_commands(word).await/})
                                }
                                
                            } 
                            
                        }
                        
                    }
                   
                },
                ServerMessage::Whisper(msg) => {
                    println!("(w) {}: {}", msg.sender.name, msg.message_text);
                },
                _ => {}
            }
        }
    });

    client.join("whoopsitspete".to_owned()).unwrap();

    // keep the tokio executor alive.
    join_handle.await.unwrap();
}
asnyc fn custom_commands(word: &str){

}