package internal

import (
	"context"
	"notifications/config"

	"github.com/go-telegram/bot"
	"github.com/go-telegram/bot/models"
)

func StartBot(ctx context.Context, state *config.App) {

	opts := []bot.Option{
		bot.WithDefaultHandler(handler),
	}

	b, err := bot.New(state.Env.TelegramAPI, opts...)
	if err != nil {
		state.Logger.Error("Telegram Bot Init Failed")
	}

	b.Start(ctx)
}

func handler(ctx context.Context, b *bot.Bot, update *models.Update) {
	b.SendMessage(ctx, &bot.SendMessageParams{
		ChatID: update.Message.Chat.ID,
		Text:   update.Message.Text,
	})

}
