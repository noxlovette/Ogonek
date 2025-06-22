package internal

import (
	"notifications/config"

	"github.com/gin-gonic/gin"
)

func NewMessageHandler(state *config.App) *MessageHandler {
	return &MessageHandler{
		state: state,
	}
}

type MessageHandler struct {
	state *config.App
}

func (h *MessageHandler) Notify(c *gin.Context) {
	var req Message

	if err := c.ShouldBindJSON(&req); err != nil {
		c.Status(522)
		return
	}
	h.state.Logger.Info(req.Message)
	c.Status(200)
}