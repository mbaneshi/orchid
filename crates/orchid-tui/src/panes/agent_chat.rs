use orchid_agent::agent::Message;
use orchid_agent::llm::LlmClient;

pub struct AgentChatPane {
    pub messages: Vec<ChatMessage>,
    pub input: String,
    pub scroll: u16,
    llm: Box<dyn LlmClient>,
    history: Vec<Message>,
}

#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

impl AgentChatPane {
    pub fn new(llm: Box<dyn LlmClient>) -> Self {
        let system_msg = Message {
            role: "system".to_string(),
            content: "You are Orchid, a helpful AI coding assistant inside a TUI workspace. \
                      Be concise and helpful."
                .to_string(),
        };
        Self {
            messages: Vec::new(),
            input: String::new(),
            scroll: 0,
            llm,
            history: vec![system_msg],
        }
    }

    pub fn push_char(&mut self, c: char) {
        self.input.push(c);
    }

    pub fn pop_char(&mut self) {
        self.input.pop();
    }

    pub async fn submit(&mut self) -> anyhow::Result<()> {
        let text = self.input.trim().to_string();
        if text.is_empty() {
            return Ok(());
        }
        self.input.clear();

        self.messages.push(ChatMessage {
            role: "user".to_string(),
            content: text.clone(),
        });

        self.history.push(Message {
            role: "user".to_string(),
            content: text,
        });

        let response = self.llm.chat(&self.history).await?;

        self.messages.push(ChatMessage {
            role: "assistant".to_string(),
            content: response.content.clone(),
        });

        self.history.push(response);
        Ok(())
    }

    pub fn scroll_up(&mut self) {
        self.scroll = self.scroll.saturating_sub(1);
    }

    pub fn scroll_down(&mut self) {
        self.scroll = self.scroll.saturating_add(1);
    }
}
