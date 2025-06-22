package main

import (
	"context"
	"log"
	"notifications/config"
	"notifications/internal"

	"github.com/gin-gonic/gin"
)

func main() {
	ctx := context.Background()
	state, err := config.InitAppState(ctx)
	if err != nil {
		log.Fatal(err)
	}

	userHandler := internal.NewMessageHandler(state)


	r := gin.Default()
	r.GET("/ping", func(c *gin.Context) {
		c.JSON(200, gin.H{"message": "pong"})
	})
	r.POST("/notify", userHandler.Notify)

	r.Run(":3000")
}