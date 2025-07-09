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
          id: "tK_H-7cD7f7mJBFvvUns0",
          title: "New Lesson",
          topic: "Meow!!!",
          assigneeName: "Kotya",
          seen: true,
          createdAt: "2025-07-03T11:03:23.362306Z",
        },
        {
          id: "pmdSfgR9zsPgiFTLrj8Vc",
          title: "Dseded?",
          topic: "General",
          assigneeName: "dseded",
          seen: true,
          createdAt: "2025-07-03T10:40:10.957506Z",
        },
        {
          id: "BpASOxIc7pqMALGPYk334",
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
      data: [
        {
          id: "task1",
          title: "Murrrrrr",
          priority: 3,
          completed: false,
          assigneeName: "Kotya",
          seen: true,
          dueDate: "2025-07-15T23:59:59Z",
        },
        {
          id: "task2",
          title: "Kotya ASsisgmned",
          priority: 2,
          completed: false,
          assigneeName: "Kotya",
          seen: true,
          dueDate: "2025-07-17T23:59:59Z",
        },
      ],
      count: 0,
    },
    decks: {
      data: [
        {
          id: "task1",
          name: "New Deck",
          assigneeName: "Kotya",
          isSubscribed: true,
          seen: true,
          visibility: "assigned",
          description: "Your New Deck",
        },
        {
          id: "dbkX_RLB8sqIcj2j2PDT3",
          name: "anew",
          assigneeName: "dseded",
          isSubscribed: false,
          seen: true,
          visibility: "assigned",
          description: "FUCK YOU!",
        },
      ],
      count: 0,
    },
    user: {
      id: "Ng4Dj62hTanaxauGWFrh1",
      name: "danila",
      username: "nox_dev",
      email: "fefefe@nfefe.com",
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
  };
  return new Response(JSON.stringify(dashboardData));
};
