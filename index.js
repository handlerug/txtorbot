const Telegraf = require('telegraf')
const GraphemeSplitter = require('grapheme-splitter')
const splitter = new GraphemeSplitter()

const randint = (min, max) => Math.floor(Math.random() * (max - min + 1) + min)

const bot = new Telegraf(process.env.TGTOKEN)

bot.start(ctx => ctx.reply(`
*Hello!* I am Textator.

I can transform your text in different ways. I caaan doooo liikee thiiis, l i k e   t h i s, or HAHAHAHAHHAH LIKE THIS!)))

Try me in inline mode!

Github repo: https://github.com/handlerug/txtorbot
`, {
    parse_mode: 'Markdown'
}))

bot.on('inline_query', ctx => {
    if (ctx.inlineQuery.query == '') {
        const result = 'e'.repeat(randint(5, 20))
        ctx.answerInlineQuery([
            {
                type: 'article',
                id: Math.random() * 100,
                title: 'eeeeee!',
                description: result,
                input_message_content: {
                    message_text: result
                }
            }
        ])
        return
    }

    const result1 = ctx.inlineQuery.query.replace(/([eе]+)/ig, (m, p, o, s) => p.repeat(randint(5, 10)))
    const result2 = ctx.inlineQuery.query.replace(/([aа]+|[yу]+|[oо]+|[eе]+)/ig, (m, p, o, s) => p.repeat(randint(5, 10)))
    const result3 = splitter.splitGraphemes(ctx.inlineQuery.query).join(' ')
    let result4 = ''
    const letters = ["А", "Х", "З", "В", "Ц", "Ф", "Щ", "Ъ", "У", "Ы"]
    for (let i = 0; i < 20; i++) {
        let min = 0
        let max = letters.length - 1
        let n = Math.floor(Math.random() * (max - min + 1)) + min
        result4 += i < 10 ? letters[n].toLowerCase() : letters[n].toUpperCase()
    }
    result4 += " " + ctx.inlineQuery.query.toUpperCase() + ")))"
    const result5 = ctx.inlineQuery.query.replace(/([eе])/ig, (m, p, o, s) => p === p.toUpperCase() ? 'Ё' : 'ё');

    ctx.answerInlineQuery([
        {
            type: 'article',
            id: Math.random() * 100,
            title: 'eeeeee!',
            description: result1,
            input_message_content: {
                message_text: result1
            }
        },
        {
            type: 'article',
            id: Math.random() * 100,
            title: 'heeeey!',
            description: result2,
            input_message_content: {
                message_text: result2
            }
        },
        {
            type: 'article',
            id: Math.random() * 100,
            title: 's p a c e d',
            description: result3,
            input_message_content: {
                message_text: result3
            }
        },
        {
            type: 'article',
            id: Math.random() * 100,
            title: 'ThunderGen by @elocurov',
            description: result4,
            input_message_content: {
                message_text: result4
            }
        },
        {
            type: 'article',
            id: Math.random() * 100,
            title: 'Для поёхавших головой',
            description: result5,
            input_message_content: {
                message_text: result5,
            },
        },
    ], {
        cache_time: 1,
        is_personal: true
    })
})
bot.startPolling()
