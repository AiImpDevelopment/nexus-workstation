/**
 * Chat Store — Conversations, streaming messages, Tauri Channel-based streaming
 * Class-based Svelte 5 pattern for shared mutable state.
 */

import { invoke, Channel } from '@tauri-apps/api/core';

export interface Message {
	id: string;
	role: 'user' | 'assistant' | 'system';
	content: string;
	timestamp: Date;
	model?: string;
	taskType?: string;
	streaming?: boolean;
}

export interface Conversation {
	id: string;
	title: string;
	messages: Message[];
	createdAt: Date;
}

type ChatEvent =
	| { event: 'Started'; data: { model: string; task_type: string } }
	| { event: 'Delta'; data: { content: string } }
	| { event: 'Finished'; data: { total_tokens: number } }
	| { event: 'Error'; data: { message: string } };

class ChatStore {
	conversations = $state<Conversation[]>([]);
	activeConversationId = $state<string | null>(null);
	isStreaming = $state(false);
	selectedModel = $state<string | null>(null);

	get activeConversation() {
		return this.conversations.find((c) => c.id === this.activeConversationId) ?? null;
	}

	get messages() {
		return this.activeConversation?.messages ?? [];
	}

	newConversation() {
		const id = crypto.randomUUID();
		this.conversations.unshift({
			id,
			title: 'New Chat',
			messages: [],
			createdAt: new Date(),
		});
		this.activeConversationId = id;
		return id;
	}

	setActive(id: string) {
		this.activeConversationId = id;
	}

	async sendMessage(content: string, openrouterKey: string) {
		if (!this.activeConversationId) this.newConversation();
		const conv = this.activeConversation!;

		conv.messages.push({
			id: crypto.randomUUID(),
			role: 'user',
			content,
			timestamp: new Date(),
		});

		const assistantMsg: Message = {
			id: crypto.randomUUID(),
			role: 'assistant',
			content: '',
			timestamp: new Date(),
			streaming: true,
		};
		conv.messages.push(assistantMsg);

		this.isStreaming = true;

		const channel = new Channel<ChatEvent>();
		channel.onmessage = (event: ChatEvent) => {
			switch (event.event) {
				case 'Started':
					assistantMsg.model = event.data.model;
					assistantMsg.taskType = event.data.task_type;
					break;
				case 'Delta':
					assistantMsg.content += event.data.content;
					break;
				case 'Finished':
					assistantMsg.streaming = false;
					this.isStreaming = false;
					if (conv.title === 'New Chat' && conv.messages.length >= 2) {
						conv.title = conv.messages[0].content.slice(0, 50);
					}
					break;
				case 'Error':
					assistantMsg.content = `Error: ${event.data.message}`;
					assistantMsg.streaming = false;
					this.isStreaming = false;
					break;
			}
		};

		try {
			await invoke('chat_stream', {
				message: content,
				modelId: this.selectedModel,
				systemPrompt: null,
				openrouterKey: openrouterKey,
				onEvent: channel,
			});
		} catch (e) {
			assistantMsg.content = `Error: ${e}`;
			assistantMsg.streaming = false;
			this.isStreaming = false;
		}
	}

	deleteConversation(id: string) {
		this.conversations = this.conversations.filter((c) => c.id !== id);
		if (this.activeConversationId === id) {
			this.activeConversationId = this.conversations[0]?.id ?? null;
		}
	}
}

export const chatStore = new ChatStore();
