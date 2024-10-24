package messages

import (
	"backend-joltamp/security"
	"github.com/gin-gonic/gin"
	"github.com/gocql/gocql"
	"net/http"
	"time"
)

type ReplyBodyType struct {
	ServerId   string
	TargetId   string
	SentAt     string
	SentAtTime time.Time
	MessageId  gocql.UUID
	Content    string
	Edited     bool
	Reactions  map[gocql.UUID]string
	SentBy     gocql.UUID
}
type Message struct {
	ServerId   string
	TargetId   string
	SentAt     string
	SentAtTime time.Time
	MessageId  gocql.UUID
	Content    string
	Edited     bool
	Reactions  map[gocql.UUID]string
	Reply      string
	SentBy     gocql.UUID
	ReplyBody  *ReplyBodyType
}

func LoadMessages(session *gocql.Session) gin.HandlerFunc {
	return func(c *gin.Context) {
		jwt := c.GetHeader("Authorization")
		if ret := security.VerifyJWT(jwt, session); ret.Status {
			var messages []Message
			var body struct {
				Target gocql.UUID  `json:"target"`
				Server *gocql.UUID `json:"server"`
				Latest *string     `json:"latest"`
			}
			if err := c.BindJSON(&body); err != nil {
				c.Status(http.StatusBadRequest)
				return
			}
			var target string
			if body.Server == nil {
				target = CombineUUIDs(ret.User.UserId, body.Target)
			} else {
				target = body.Target.String()
			}
			var server string
			if body.Server == nil {
				server = ""
			} else {
				server = body.Server.String()
			}
			var iter *gocql.Iter
			if body.Latest != nil {
				latest, _ := time.Parse(time.RFC3339, *body.Latest)
				println(latest.Format("2006-01-02 15:04:05.999"))
				iter = session.Query(`SELECT * FROM messages WHERE target_id = ? AND server_id = ? AND sent_at_time > ? LIMIT 50`, target, server, latest.Format("2006-01-02 15:04:05.999")).Iter()
			} else {
				iter = session.Query(`SELECT * FROM messages WHERE target_id = ? AND server_id = ? LIMIT 50`, target, server).Iter()
			}
			for {
				var msg Message

				// Scan the current row into msg
				if !iter.Scan(&msg.ServerId, &msg.TargetId, &msg.SentAt, &msg.SentAtTime, &msg.MessageId, &msg.Content, &msg.Edited, &msg.Reactions, &msg.Reply, &msg.SentBy) {
					break
				}
				if msg.Reply != "" {
					if msg.ReplyBody == nil {
						msg.ReplyBody = &ReplyBodyType{}
					}
					if err := session.Query(`SELECT server_id, target_id, sent_at, sent_at_time, message_id, content, edited, reactions, sent_by FROM messages WHERE target_id = ? AND server_id = ? AND message_id = ? ALLOW FILTERING`, target, server, msg.Reply).Scan(
						&msg.ReplyBody.ServerId,
						&msg.ReplyBody.TargetId,
						&msg.ReplyBody.SentAt,
						&msg.ReplyBody.SentAtTime,
						&msg.ReplyBody.MessageId,
						&msg.ReplyBody.Content,
						&msg.ReplyBody.Edited,
						&msg.ReplyBody.Reactions,
						&msg.ReplyBody.SentBy); err != nil {
						println(err.Error())
						c.JSON(http.StatusInternalServerError, err.Error())
					}
				}
				messages = append(messages, msg)
			}
			c.JSON(http.StatusOK, messages)
		} else {
			c.JSON(http.StatusBadRequest, gin.H{"error": "Bad JWT token!"})
			return
		}
	}
}
