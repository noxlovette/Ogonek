import { tasks } from "$lib/server/mock";
import type { DashboardData } from "$lib/types";

export const GET = async () => {
  const dashboardData: DashboardData = {
    students: [
      {
        id: "rLaGXZNx9BDZphe201odc",
        name: "dseded",
        username: "dseded",
        email: "dseded@h.ru",
        markdown: null,
        studentTelegramId: null,
      },
      {
        id: "ikqO0Lwr7ZH-hdk5f07WV",
        name: "Kotya",
        username: "kotya",
        email: "kotya@iclud.com",
        markdown: null,
        studentTelegramId: null,
      },
    ],
    lessons: {
      data: [
        {
          id: "lesson1",
          title: "New Lesson",
          topic: "Meow!!!",
          assigneeName: "Kotya",
          seen: true,
          createdAt: "2025-07-03T11:03:23.362306Z",
        },
        {
          id: "lesson2",
          title: "Dseded?",
          topic: "General",
          assigneeName: "dseded",
          seen: true,
          createdAt: "2025-07-03T10:40:10.957506Z",
        },
        {
          id: "lesson3",
          title: "Kotya!!!",
          topic: "Badges Test 3",
          assigneeName: "Kotya",
          seen: true,
          createdAt: "2025-07-03T10:36:38.337548Z",
        },
      ],
      count: 0,
    },
    tasks: {
      data: tasks,
      count: 0,
    },
    decks: {
      data: [
        {
          id: "task1",
          name: "Water the Flowers",
          assigneeName: "Sasha",
          isSubscribed: true,
          seen: false,
          visibility: "assigned",
          description: "love; 2046",
        },
        {
          id: "dbkX_RLB8sqIcj2j2PDT3",
          name: "Start Anew",
          assigneeName: "dseded",
          isSubscribed: false,
          seen: true,
          visibility: "assigned",
          description: "sacred; duty",
        },
      ],
      count: 0,
    },
    user: {
      id: "Ng4Dj62hTanaxauGWFrh1",
      name: "Michael",
      username: "michael_developer",
      email: "hahaha@icloud.com",
      role: "teacher",
    },
    profile: {
      profile: {
        userId: "Ng4Dj62hTanaxauGWFrh1",
        videoCallUrl: null,
        avatarUrl: null,
        telegramId: null,
      },
      teacherData: null,
    },
    learn: {
      dueCards: 10,
      stats: {
        cardsStudiedToday: 3,
        currentSterak: 5,
      },
    },
    preferences: {
      auto_subscribe: true,
      email_notifications: false,
      push_notifications: false,
      theme: "system",
      language: "en",
    },
    activity: [
      {
        modelType: "lesson",
        modelId: "lesson1",
        action: "updated",
        createdAt: new Date().toISOString(),
      },
      {
        modelType: "deck",
        modelId: "lesson1",
        action: "updated",
        createdAt: new Date(Date.now() - 300000).toISOString(), // 5 min ago
      },
      {
        modelType: "task",
        modelId: "lesson1",
        action: "updated",
        createdAt: new Date(Date.now() - 600000).toISOString(), // 10 min ago
      },
    ],
  };
  return new Response(JSON.stringify(dashboardData));
};
