use teloxide::{prelude::*, utils::command::BotCommands};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Command list:")]
enum Command {
    #[command(description = "Вывести этот текст")]
    Help,
    #[command(description = "Получить ссылку на сайт")]
    GetLink,
    #[command(description = "Верификация другим путём")]
    AnotherWay,
}

#[tokio::main]
async fn main() {
    println!("INFO: Starting Anti's Bot");
    let bot = Bot::from_env();
    let start_message_sender = bot.clone();

    teloxide::repl(start_message_sender, |bot: Bot, msg: Message| async move {
        bot.send_message(
            msg.chat.id,
            "Привет! Прежде чем ты попадёшь на сайт, мы должны убедится, 
            что ты сделан не из того же, из чего и я. 
            Пожалуйста пройди по ссылка и подтверди, что ты человек. <ссылка>.\n
            Список доступных комманд можно посмотреть введя /help",
        )
        .await?;
        Ok(())
    })
    .await;

    Command::repl(bot, answer).await;
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::GetLink => {
            bot.send_message(msg.chat.id, "Сайт ещё не родился".to_string())
                .await?
        }
        Command::AnotherWay => {
            bot.send_message(
                msg.chat.id,
                "Данная опция не поддерживается на нынешнем этапе разработки".to_string(),
            )
            .await?
        }
    };

    Ok(())
}
