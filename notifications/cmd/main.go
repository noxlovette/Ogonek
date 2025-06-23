package main

import (
	"context"
	"log"
	"notifications/config"
	"notifications/internal"
	"os"
	"os/signal"
	"syscall"

	"github.com/gin-gonic/gin"
)

func main() {
	ctx, stop := signal.NotifyContext(context.Background(), os.Interrupt, syscall.SIGTERM)
	defer stop()

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

	go internal.StartBot(ctx, state)
	go func() {
		if err := r.Run(":3000"); err != nil {
			log.Fatalf("Gin server failed: %v", err)
		}
	}()

	<-ctx.Done() // Wait for Ctrl+C or SIGTERM
	log.Println("Shutting down gracefully...")
}
