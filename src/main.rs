use rand::prelude::*;
use std::cmp::min;
use std::sync::Arc;
use teloxide::prelude::*;
use teloxide::requests::ResponseResult;
use teloxide::types::{
    InlineQueryResult, InlineQueryResultArticle, InputMessageContent, InputMessageContentText,
    ParseMode,
};
use tokio_stream::wrappers::UnboundedReceiverStream;
use unicode_segmentation::UnicodeSegmentation;

// This is the welcome message that's sent to users when they interact with the bot im PMs.
// Remember to disable groups in BotFather and make sure privacy mode is on! This bot works
// only in inline mode, and it'll reply to every message with the text below.
const WELCOME_MESSAGE: &str = "\
Hi! I transform text. Ever felt the need to  e m p h a s i z e  something with more spaces? Or \
perhaps you take it eeeeeeeeeasy? Wanna MoCk SoMeOnE? I do the work of pressing Space and Shift \
for you.

Just type <code>@txtorbot</code> and the text you want to transform. There are many transformers \
available, with more to come.

Source code: https://github.com/handlerug/txtorbot";

#[tokio::main]
async fn main() {
    // Errors are less cryptic this way.
    run().await;
}

async fn run() {
    // Parse emojis from the disk. Reading is done at compile time, but parsing is done at runtime.
    let emojis = include_str!("../emoji.txt");
    let emojis = Arc::new(emojis.split(' ').collect::<Vec<&str>>());

    teloxide::enable_logging!();
    log::info!("Starting txtorbot...");

    // Initialize the bot with parameters from the environment. TELOXIDE_TOKEN is required!
    let bot = Bot::from_env();

    // Start a dispatcher that'll poll for updates and push new ones to the queues.
    Dispatcher::new(bot)
        .messages_handler(|rx: DispatcherHandlerRx<Bot, Message>| {
            UnboundedReceiverStream::new(rx).for_each_concurrent(None, |query| async move {
                // log_on_error() writes to log when an API error happens. Make sure to return
                // teloxide's ResponseResult from the handler for it to work!
                handle_message(query).await.log_on_error().await
            })
        })
        .inline_queries_handler(move |rx: DispatcherHandlerRx<Bot, InlineQuery>| {
            UnboundedReceiverStream::new(rx).for_each_concurrent(None, move |query| {
                // The handler can run asynchronously, so we are using atomic reference counting
                // mechanism here. It keeps count by incrementing the counter when it's cloned,
                // and decrementing when the cloned reference is destroyed. Arc is read-only,
                // and it's Send and Sync, so we can use the cloned reference from any thread.
                let emojis = Arc::clone(&emojis);
                async move {
                    handle_inline_query(query, emojis)
                        .await
                        .log_on_error()
                        .await
                }
            })
        })
        .dispatch()
        .await;
}

/// Handles incoming messages and
async fn handle_message(query: UpdateWithCx<Bot, Message>) -> ResponseResult<()> {
    query
        .answer(WELCOME_MESSAGE)
        .parse_mode(ParseMode::Html)
        .send()
        .await?;

    // respond(()) is a shortcut for ResponseResult::Ok(()).
    respond(())
}

async fn handle_inline_query(
    query: UpdateWithCx<Bot, InlineQuery>,
    emojis: Arc<Vec<&str>>,
) -> ResponseResult<()> {
    let InlineQuery {
        id, query: text, ..
    } = query.update;

    if text.is_empty() {
        return respond(());
    }

    let results = vec![
        article_from_text("Eeeify", &eeify(&text)),
        article_from_text("Vowelify", &vowelify(&text)),
        article_from_text("Spaced", &spaced(&text)),
        article_from_text("FeNcIfY", &fencify(&text)),
        article_from_text("Emoji", &emojify(&text, emojis)),
        article_from_text("ThunderGen", &thunder_gen(&text)),
        article_from_text("Cyrillic ё", &yoify(&text)),
    ];

    query
        .requester
        .answer_inline_query(id, results)
        .cache_time(1)
        .send()
        .await?;
    respond(())
}

/// Creates InlineQueryResult::Article with specified title and content. It also automatically
/// truncates the content to prevent errors.
fn article_from_text(title: &str, content: &str) -> InlineQueryResult {
    let content = truncate(content, 4096);

    // Concatenate all the text we have.
    let mut text = String::with_capacity(title.len() + content.len());
    text.push_str(title);
    text.push_str(content);

    // Hash the result to make a unique ID. seahash claims to be fast.
    let id = format!("{:016x}", seahash::hash(text.as_bytes()));
    let description = truncate(content, 250); // 250 is an arbitrary number
    let input = InputMessageContent::Text(InputMessageContentText::new(content.to_owned()));

    InlineQueryResult::Article(
        InlineQueryResultArticle::new(id, title, input).description(description),
    )
}

/// Calls the predicate for every character and, if it returns true, prolongs them.
fn prolong<P: Fn(&str) -> bool>(input: &str, predicate: P) -> String {
    // Belongs to [3; 9]
    let amount: usize = 3 + min((input.len() as f64 / 256.0 * 32.0) as usize, 6usize);
    let mut result = String::with_capacity((input.len() as f64 * 1.5) as usize);

    for grapheme in input.graphemes(true) {
        if predicate(grapheme) {
            result.push_str(&grapheme.to_string().repeat(amount));
        } else {
            result.push_str(grapheme);
        }
    }

    result
}

/// Prolongs Latin and Cyrillic e.
fn eeify(input: &str) -> String {
    prolong(input, |c| {
        let c = c.to_lowercase();
        // Latin and Cyrillic e
        c == "e" || c == "е"
    })
}

/// Prolongs every Latin and Cyrillic vowel.
fn vowelify(input: &str) -> String {
    let test = "aeiouyаяоёэеуюыи";
    prolong(input, |c| test.contains(c))
}

/// Adds spaces between characters.
fn spaced(input: &str) -> String {
    let mut result = String::with_capacity(input.len() * 2);

    // Used for de-duplicating spaces. Duplicated spaces break the vibe.
    let mut was_space = false;

    for grapheme in input.graphemes(true) {
        if grapheme == " " {
            if was_space {
                continue;
            }
            was_space = true;
        } else {
            was_space = false;
        }

        result.push_str(grapheme);
        result.push(' ');
    }
    result
}

/// Makes every even character uppercase and every odd one lowercase. LiKe ThIs.
fn fencify(input: &str) -> String {
    let mut i = 0;
    let mut result = String::with_capacity(input.len());

    for grapheme in input.graphemes(true) {
        if i % 2 == 0 {
            result.push_str(&grapheme.to_uppercase());
        } else {
            result.push_str(&grapheme.to_lowercase());
        }

        if grapheme != " " {
            i += 1;
        }
    }

    result
}

/// Inserts a random emoji between all words.
fn emojify(input: &str, emojis: Arc<Vec<&str>>) -> String {
    let hash = seahash::hash(input.as_bytes());
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(hash);
    let mut result = String::with_capacity(input.len() * 2);

    for word in input.unicode_words() {
        result.push_str(word);
        result.push(' ');
        result.push_str(emojis.choose(&mut rng).unwrap());
        result.push(' ');
    }

    result
}

/// ThunderGen
fn thunder_gen(input: &str) -> String {
    // original algorithm written in JavaScript by @elocurov

    let input = input
        .to_uppercase()
        .unicode_words()
        .collect::<Vec<&str>>()
        .join(" ");

    let lowercase = ['а', 'х', 'з', 'в', 'ц', 'ф', 'щ', 'ъ', 'у', 'ы'];
    let uppercase = ['А', 'Х', 'З', 'В', 'Ц', 'Ф', 'Щ', 'Ъ', 'У', 'Ы'];
    let hash = seahash::hash(input.as_bytes());
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(hash);
    let mut result = String::with_capacity(44 + input.len());

    for i in 0..20usize {
        let letter = if i < 10 {
            lowercase.choose(&mut rng).unwrap()
        } else {
            uppercase.choose(&mut rng).unwrap()
        };
        result.push(*letter);
    }

    result + " " + &input + ")))"
}

/// Replace Latin and Cyrillic e with Cyrillic ё.
fn yoify(input: &str) -> String {
    let mut result = String::with_capacity(input.len());

    for grapheme in input.graphemes(true) {
        // Latin and Cyrillic e
        if grapheme == "e" || grapheme == "е" {
            result.push('ё');
        } else if grapheme == "E" || grapheme == "Е" {
            result.push('Ё');
        } else {
            result.push_str(grapheme);
        }
    }

    result
}

/// Truncates a string to have at most len bytes.
fn truncate(input: &str, len: usize) -> &str {
    if input.len() <= len {
        return input;
    }

    let chars = input.char_indices().map(|a| a.0).collect::<Vec<usize>>();
    let mut left = 0usize;
    let mut right = chars.len() - 1;

    while left + 1 < right {
        let mid = (left + right) / 2;
        let idx = chars[mid];

        if idx < len {
            left = mid;
        } else {
            right = mid;
        }
    }

    &input[0..chars[right]]
}

#[cfg(test)]
mod tests {
    use crate::truncate;

    #[test]
    fn test_truncate() {
        let too_short = "a".repeat(1000);
        let just_enough = "a".repeat(4096);
        let too_long = "a".repeat(4100);
        let tfw_unicode = "a".repeat(4095) + "ы";

        assert_eq!(truncate(&too_short, 4096).len(), 1000, "too short");
        assert_eq!(
            truncate(&just_enough, 4096).len(),
            4096,
            "exactly 4096 bytes"
        );
        assert_eq!(truncate(&too_long, 4096).len(), 4096, "too long");
        assert_eq!(
            truncate(&tfw_unicode, 4096).len(),
            4095,
            "4097 bytes with a codepoint lying on the boundary"
        );
    }
}
