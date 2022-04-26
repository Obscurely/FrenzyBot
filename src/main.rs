mod openai;
mod discord;
use discord::discord::Discord;
use openai::davinci;
use std::thread;
use std::time;
use reqwest;
use tokio::task;

#[tokio::main]
async fn main() {
    let ai =
        davinci::Davinci::from("".to_string());
    let dc = Discord::from("".to_string()).await.unwrap();

    let mess_raw = dc.get_10_mess_in_dm(605429245631660067).await;
    let mess = discord::util::format_mess_for_ai(&mess_raw, "Obscurely");

    println!("{}", mess);
     

    // let input = "Raspunde doar cu un mesaj:\n\
    //         Prieten: Aha\n\
    //         Prieten: Ok\n\
    //         Prieten: În lumea ta face logica\n\
    //         Prieten: Nu am sa te contrazic\n\
    //         Eu: inca e in beta, mai am planuri cu el\n\
    //         Eu: in lumea linux-ului\n\
    //         Eu: si partea cea mai buna e ca trece de verificari de robot si dai bypass la spam, practic il poti folosi ca ddos tool\n\
    //         Eu: nu ca ar fi intentionat...\n\
    //         Prieten: Aha\n\
    //         Eu:
    //     ";
    //
    let mut previous_mess = String::from("");
    loop {
        println!("Sleeping...");
        thread::sleep(time::Duration::from_secs(5));
        let mess_raw = dc.get_10_mess_in_dm(605429245631660067).await;
        let mess = discord::util::format_mess_for_ai(&mess_raw, "Obscurely");
        if mess != previous_mess {
            let res = ai.complete_str(&mess).await.unwrap();
            dc.send_mess_in_dm(res, 605429245631660067).await;
            let mess_raw = dc.get_10_mess_in_dm(605429245631660067).await;
            let mess = discord::util::format_mess_for_ai(&mess_raw, "Obscurely");
            // println!("{}", res);
            // println!("{}", mess);
            previous_mess = mess.clone();
        }
    }
}
