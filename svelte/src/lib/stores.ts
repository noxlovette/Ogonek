import { writable } from 'svelte/store';
import type { User } from './types';

export const user = writable({
	username: '',
	name: '',
	// quizlet_url: '',
});

export function setUser(data: User) {
	user.update((currentState) => ({
		...currentState,
		...data
	}));
}

export function clearUser() {
	user.update(() => ({
		username: '',
		name:''
		// quizlet_url: '',
	}));
}

export const notification = writable({
	message: '',
	type: 'none'
});

export const lesson = writable({
	title: '',
	description: '',
	bookmarked: false,
	manual_date: '',
	content: '',
	created_at: '',
	updated_at: '',
	topic: ''
});

export const language = writable('en');

export const translations = writable({
	lesson: {
		en: 'Lesson',
		de: 'Lektion',
		ru: 'Урок'
	},
	settings: {
		en: 'Settings',
		de: 'Einstellungen',
		ru: 'Настройки'
	},
	bookmarks: {
		en: 'Bookmarks',
		de: 'Lesezeichen',
		ru: 'Закладки'
	},
	search: {
		en: 'Search lessons',
		de: 'Lektionen suchen',
		ru: 'Поиск уроков'
	},
	tasks: {
		en: 'Tasks',
		de: 'Aufgaben',
		ru: 'Задачи'
	},
	bookmark: {
		en: 'Bookmark',
		de: 'Lesezeichen hinzufügen',
		ru: 'Добавить в закладки'
	},
	unbookmark: {
		en: 'Unbookmark',
		de: 'Lesezeichen entfernen',
		ru: 'Удалить из закладок'
	},
	bookmark_added: {
		en: 'Bookmark added',
		de: 'Lesezeichen hinzugefügt',
		ru: 'Закладка добавлена'
	},
	recent: {
		en: 'Recent Lessons',
		de: 'Neueste Lektionen',
		ru: 'Последние уроки'
	},
	view_all: {
		en: 'View all',
		de: 'Alle anzeigen',
		ru: 'Показать все'
	},
	view_bookmarked: {
		en: 'View bookmarked',
		de: 'Lesezeichen anzeigen',
		ru: 'Показать закладки'
	},
	useful_links: {
		en: 'Useful links',
		de: 'Nützliche Links',
		ru: 'Полезные ссылки'
	},
	bookmarked: {
		en: 'Bookmarked',
		de: 'Lesezeichen',
		ru: 'Закладки'
	},
	topic: {
		en: 'Topic',
		de: 'Thema',
		ru: 'Тема'
	},
	category: {
		en: 'Category',
		de: 'Kategorie',
		ru: 'Категория'
	},
	random_word: {
		en: 'Random word',
		de: 'Zufälliges Wort',
		ru: 'Случайное слово'
	},
	dashboard: {
		en: 'Dashboard',
		de: 'Dashboard',
		ru: 'Домой'
	},
	profile: {
		en: 'Profile',
		de: 'Profil',
		ru: 'Профиль'
	},
	logout: {
		en: 'Logout',
		de: 'Abmelden',
		ru: 'Выйти'
	},
	login: {
		en: 'Login',
		de: 'Anmelden',
		ru: 'Войти'
	},
	dictionary: {
		en: 'Dictionary',
		de: 'Wörterbuch',
		ru: 'Словарь'
	},
	thesaurus: {
		en: 'Thesaurus',
		de: 'Thesaurus',
		ru: 'Тезаурус'
	},
	pronunciation: {
		en: 'Pronunciation',
		de: 'Aussprache',
		ru: 'Произношение'
	},
	text_me: {
		en: 'Text Danila',
		de: 'Danila Texten',
		ru: 'Написать Даниле'
	},
	no_bookmarks: {
		en: 'No bookmarks yet',
		de: 'Noch keine Lesezeichen',
		ru: 'Пока нет закладок'
	},
	lessons: {
		en: 'Lessons',
		de: 'Lektionen',
		ru: 'Уроки'
	},

	no_tasks: {
		en: 'No tasks yet',
		de: 'Noch keine Aufgaben',
		ru: 'Пока нет задач'
	},
	no_lessons: {
		en: 'No lessons yet',
		de: 'Noch keine Lektionen',
		ru: 'Пока нет уроков'
	},
	tasks_completed: {
		en: 'Show completed',
		de: 'Erledigte anzeigen',
		ru: 'Показать выполненные'
	},
	tasks_notcompleted: {
		en: 'Hide completed',
		de: 'Erledigte verstecken',
		ru: 'Скрыть невыполненные'
	},
	recent_activity: {
		en: 'Recent activity',
		de: 'Neueste Aktivität',
		ru: 'Последняя активность'
	},
	evening: {
		en: 'Good evening',
		de: 'Guten Abend',
		ru: 'Добрый вечер'
	},
	morning: {
		en: 'Good morning',
		de: 'Guten Morgen',
		ru: 'Доброе утро'
	},
	afternoon: {
		en: 'Good afternoon',
		de: 'Guten Tag',
		ru: 'Добрый день'
	},
	night: {
		en: 'Good night',
		de: 'Gute Nacht',
		ru: 'Спокойной ночи'
	},
	new_lesson: {
		en: 'New lesson',
		de: 'Neue Lektion',
		ru: 'Новый урок'
	},
	new_tasks: {
		en: 'New tasks',
		de: 'Neue Aufgaben',
		ru: 'Новые задачи'
	}
});
