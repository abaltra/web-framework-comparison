package main

import (
	"encoding/json"
	"net/http"
	"os"

	"github.com/savsgio/atreugo/v11"
)

func main() {

	config := atreugo.Config{
		Addr: "0.0.0.0:3001",
	}

	if os.Getenv("USE_TLS") != "" {
		config.TLSEnable = true
		config.CertFile = "../certs/cert.crt"
		config.CertKey = "../certs/key.key"
	}

	server := atreugo.New(config)

	server.GET("/json", func(ctx *atreugo.RequestCtx) error {
		body := map[string]string{"value": "value"}
		return ctx.JSONResponse(body, http.StatusOK)
	})

	server.GET("/text", func(ctx *atreugo.RequestCtx) error {
		return ctx.TextResponse("value", http.StatusOK)
	})

	server.POST("/body", func(ctx *atreugo.RequestCtx) error {
		var j map[string]string
		json.Unmarshal(ctx.Request.Body(), &j)
		return ctx.JSONResponse(j, http.StatusOK)
	})

	if err := server.ListenAndServe(); err != nil {
		panic(err)
	}
}
