use teloxide::prelude::*;
use teloxide::types::*;
use teloxide::requests::JsonRequest;
use teloxide::payloads::AnswerInlineQuery;
use tokio_stream::wrappers::UnboundedReceiverStream;

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting txtorbot...");

    let bot = Bot::from_env();

    Dispatcher::new(bot)
        .inline_queries_handler(|rx: DispatcherHandlerRx<Bot, InlineQuery>| {
            UnboundedReceiverStream::new(rx).for_each_concurrent(None, move |query| async {
                let InlineQuery { id, query: text, .. } = query.update;

                let title = String::from("Title");
                let content = String::from("Some text");

                let mut text = String::with_capacity(title.len() + content.len());
                text.push_str(&title);
                text.push_str(&content);

                let result = InlineQueryResult::Article(InlineQueryResultArticle {
                    id: format!("{:016x}", seahash::hash(text.as_bytes())),
                    title,
                    input_message_content: InputMessageContent::Text(InputMessageContentText {
                        message_text: content.clone(),
                        parse_mode: None,
                        entities: None,
                        disable_web_page_preview: None
                    }),
                    description: Some(content),
                    reply_markup: None,
                    url: None,
                    hide_url: None,
                    thumb_url: None,
                    thumb_width: None,
                    thumb_height: None
                });

                let mut payload = AnswerInlineQuery::new(id, vec![result]);
                payload.cache_time = Some(1);

                let request = JsonRequest::new(query.requester, payload);
                request.send().await.log_on_error().await;
                ()
            })
        })
        .dispatch()
        .await;
}