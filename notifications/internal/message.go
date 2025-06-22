package internal

type Message struct {
	TelegramID string `json:"telegram_id"`
	Username string `json:"username"`
	PermissionBrowser bool `json:"permission_browser"`
	Message string `json:"message"`
}