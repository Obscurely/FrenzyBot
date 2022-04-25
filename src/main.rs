mod openai;
use openai::davinci;
use reqwest;
use tokio::task;

#[tokio::main]
async fn main() {
    let ai =
        davinci::Davinci::from("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".to_string());

    let input = "Raspunde doar cu un mesaj:\n\
            Prieten: Aha\n\
            Prieten: Ok\n\
            Prieten: În lumea ta face logica\n\
            Prieten: Nu am sa te contrazic\n\
            Eu: inca e in beta, mai am planuri cu el\n\
            Eu: in lumea linux-ului\n\
            Eu: si partea cea mai buna e ca trece de verificari de robot si dai bypass la spam, practic il poti folosi ca ddos tool\n\
            Eu: nu ca ar fi intentionat...\n\
            Prieten: Aha\n\
            Eu:
        ";

    println!("{}", input);

    let res = ai.complete_str(input).await;
    println!("{}", res.unwrap());
}
