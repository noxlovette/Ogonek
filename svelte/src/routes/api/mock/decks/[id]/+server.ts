import logger from "$lib/logger";
import type { DeckWithCards } from "$lib/types";
import type { RequestHandler } from "./$types";

export const GET: RequestHandler = async ({ params }) => {
  let deck: DeckWithCards | null = null;

  switch (params.id) {
    case "deck1":
      deck = {
        deck: {
          id: "deck1",
          name: "New Deck",
          description: "Your New Deck",
          visibility: "assigned",
          assignee: "ikqO0Lwr7ZH-hdk5f07WV",
          isSubscribed: true,
          createdBy: "Ng4Dj62hTanaxauGWFrh1",
          createdAt: "2025-07-03T07:09:59.153087Z",
        },
        cards: [
          {
            id: "bPLmTFB1aXXinsVM9rblo",
            front: "hi",
            back: "buy",
            mediaUrl: null,
            deckId: "hpSL740Zl-Mzqo4m6m1mY",
            createdAt: "2025-07-04T07:14:34.508007Z",
          },
          {
            id: "0z0bywhCdZhMtnGaCq_cF",
            front: "fefef",
            back: "poihfo;eiafwe",
            mediaUrl: null,
            deckId: "hpSL740Zl-Mzqo4m6m1mY",
            createdAt: "2025-07-04T07:14:34.508007Z",
          },
          {
            id: "4e8jiLJCmvVYfPrKXhzce",
            front: "fewfew",
            back: "fefefe",
            mediaUrl: null,
            deckId: "hpSL740Zl-Mzqo4m6m1mY",
            createdAt: "2025-07-04T07:14:34.508007Z",
          },
          {
            id: "wzg-cWMGwridb0QvUoSWE",
            front: "air conditioning",
            back: "A system used to cool the air inside buildings or vehicles, especially in hot weather.",
            mediaUrl: null,
            deckId: "hpSL740Zl-Mzqo4m6m1mY",
            createdAt: "2025-07-04T07:14:34.508007Z",
          },
          {
            id: "2o94lGyzkJxQP0ayy5wue",
            front: "bus driver",
            back: "A person whose job is to drive a bus and transport passengers along a set route.",
            mediaUrl: null,
            deckId: "hpSL740Zl-Mzqo4m6m1mY",
            createdAt: "2025-07-04T07:14:34.508007Z",
          },
          {
            id: "u3jo9FYwmU9Ly1UJK5PYe",
            front: "culture shock",
            back: "A feeling of confusion or stress experienced when someone is suddenly exposed to a very different culture or way of life.",
            mediaUrl: null,
            deckId: "hpSL740Zl-Mzqo4m6m1mY",
            createdAt: "2025-07-04T07:14:34.508007Z",
          },
          {
            id: "sJIObuD-VXUFjbP3ArT_2",
            front: "cycle lane",
            back: "A designated part of a road marked for use by people riding bicycles.",
            mediaUrl: null,
            deckId: "hpSL740Zl-Mzqo4m6m1mY",
            createdAt: "2025-07-04T07:14:34.508007Z",
          },
          {
            id: "M7LMerM98rrWuVGzdwo2S",
            front: "firefighter",
            back: "A person trained to extinguish fires and rescue people from dangerous situations.",
            mediaUrl: null,
            deckId: "hpSL740Zl-Mzqo4m6m1mY",
            createdAt: "2025-07-04T07:14:34.508007Z",
          },
          {
            id: "rksMtDxxISLSgtLWrDhpX",
            front: "forest fires",
            back: "Uncontrolled fires that spread rapidly through wild or wooded areas, often causing damage to wildlife and property.",
            mediaUrl: null,
            deckId: "hpSL740Zl-Mzqo4m6m1mY",
            createdAt: "2025-07-04T07:14:34.508007Z",
          },
          {
            id: "0Wt80q3EbkUTJD3GSahXh",
            front: "living conditions",
            back: "The circumstances affecting the way people live, such as housing, sanitation, safety, and access to resources.",
            mediaUrl: null,
            deckId: "hpSL740Zl-Mzqo4m6m1mY",
            createdAt: "2025-07-04T07:14:34.508007Z",
          },
          {
            id: "0bm4IYfSMc6_RuTeghTIR",
            front: "pedestrian crossing",
            back: "A marked area on a road where people are allowed to walk across, usually with priority over vehicles.",
            mediaUrl: null,
            deckId: "hpSL740Zl-Mzqo4m6m1mY",
            createdAt: "2025-07-04T07:14:34.508007Z",
          },
          {
            id: "cQMi582DBFxscg9dkG3h2",
            front: "rush hour",
            back: "The busiest time of day on the roads and public transport, typically when people are traveling to or from work.",
            mediaUrl: null,
            deckId: "hpSL740Zl-Mzqo4m6m1mY",
            createdAt: "2025-07-04T07:14:34.508007Z",
          },
          {
            id: "bQ3VKkGQRIeV-v4FwwZ39",
            front: "traffic jam",
            back: "A situation where vehicles are unable to move freely because of heavy congestion.",
            mediaUrl: null,
            deckId: "hpSL740Zl-Mzqo4m6m1mY",
            createdAt: "2025-07-04T07:14:34.508007Z",
          },
          {
            id: "fSE9xUjddVhBDp4ww_6ZE",
            front: "traffic lights",
            back: "Electrically operated signals at road intersections that control the flow of traffic using red, yellow, and green lights.",
            mediaUrl: null,
            deckId: "hpSL740Zl-Mzqo4m6m1mY",
            createdAt: "2025-07-04T07:14:34.508007Z",
          },
          {
            id: "7s2j7dYimr7DA9kX8zMp_",
            front: "workplace",
            back: "The location or environment where a person performs their job or profession.",
            mediaUrl: null,
            deckId: "hpSL740Zl-Mzqo4m6m1mY",
            createdAt: "2025-07-04T07:14:34.508007Z",
          },
          {
            id: "W1ja-HOPa9GEduqBY2aZW",
            front:
              "The location or environment where a person performs their job or profession.",
            back: "The location or environment where a person performs their job or profession.",
            mediaUrl: null,
            deckId: "hpSL740Zl-Mzqo4m6m1mY",
            createdAt: "2025-07-04T07:14:34.508007Z",
          },
          {
            id: "LD3a4SWEnm1t5vacCjAX3",
            front:
              "The location or environment where a person performs their job or profession.",
            back: "The location or environment where a person performs their job or profession.",
            mediaUrl: null,
            deckId: "hpSL740Zl-Mzqo4m6m1mY",
            createdAt: "2025-07-04T07:14:34.508007Z",
          },
          {
            id: "IOpGx6ThhkIAH9TQ0kCS6",
            front:
              "The location or environment where a person performs their job or profession.",
            back: "The location or environment where a person performs their job or profession.",
            mediaUrl: null,
            deckId: "hpSL740Zl-Mzqo4m6m1mY",
            createdAt: "2025-07-04T07:14:34.508007Z",
          },
        ],
        isSubscribed: true,
      };
      break;
    case "deck2":
      deck = {
        deck: {
          id: "deck2",
          name: "New Deck",
          description: "Your New Deck",
          visibility: "assigned",
          assignee: "ikqO0Lwr7ZH-hdk5f07WV",
          isSubscribed: true,
          createdBy: "Ng4Dj62hTanaxauGWFrh1",
          createdAt: "2025-07-03T07:09:59.153087Z",
        },
        cards: [
          {
            id: "bPLmTFB1aXXinsVM9rblo",
            front: "hi",
            back: "buy",
            mediaUrl: null,
            deckId: "hpSL740Zl-Mzqo4m6m1mY",
            createdAt: "2025-07-04T07:14:34.508007Z",
          },
          {
            id: "0z0bywhCdZhMtnGaCq_cF",
            front: "fefef",
            back: "poihfo;eiafwe",
            mediaUrl: null,
            deckId: "hpSL740Zl-Mzqo4m6m1mY",
            createdAt: "2025-07-04T07:14:34.508007Z",
          },
          {
            id: "4e8jiLJCmvVYfPrKXhzce",
            front: "fewfew",
            back: "fefefe",
            mediaUrl: null,
            deckId: "hpSL740Zl-Mzqo4m6m1mY",
            createdAt: "2025-07-04T07:14:34.508007Z",
          },
          {
            id: "wzg-cWMGwridb0QvUoSWE",
            front: "air conditioning",
            back: "A system used to cool the air inside buildings or vehicles, especially in hot weather.",
            mediaUrl: null,
            deckId: "hpSL740Zl-Mzqo4m6m1mY",
            createdAt: "2025-07-04T07:14:34.508007Z",
          },
          {
            id: "2o94lGyzkJxQP0ayy5wue",
            front: "bus driver",
            back: "A person whose job is to drive a bus and transport passengers along a set route.",
            mediaUrl: null,
            deckId: "hpSL740Zl-Mzqo4m6m1mY",
            createdAt: "2025-07-04T07:14:34.508007Z",
          },
          {
            id: "u3jo9FYwmU9Ly1UJK5PYe",
            front: "culture shock",
            back: "A feeling of confusion or stress experienced when someone is suddenly exposed to a very different culture or way of life.",
            mediaUrl: null,
            deckId: "hpSL740Zl-Mzqo4m6m1mY",
            createdAt: "2025-07-04T07:14:34.508007Z",
          },
          {
            id: "sJIObuD-VXUFjbP3ArT_2",
            front: "cycle lane",
            back: "A designated part of a road marked for use by people riding bicycles.",
            mediaUrl: null,
            deckId: "hpSL740Zl-Mzqo4m6m1mY",
            createdAt: "2025-07-04T07:14:34.508007Z",
          },
          {
            id: "M7LMerM98rrWuVGzdwo2S",
            front: "firefighter",
            back: "A person trained to extinguish fires and rescue people from dangerous situations.",
            mediaUrl: null,
            deckId: "hpSL740Zl-Mzqo4m6m1mY",
            createdAt: "2025-07-04T07:14:34.508007Z",
          },
          {
            id: "rksMtDxxISLSgtLWrDhpX",
            front: "forest fires",
            back: "Uncontrolled fires that spread rapidly through wild or wooded areas, often causing damage to wildlife and property.",
            mediaUrl: null,
            deckId: "hpSL740Zl-Mzqo4m6m1mY",
            createdAt: "2025-07-04T07:14:34.508007Z",
          },
          {
            id: "0Wt80q3EbkUTJD3GSahXh",
            front: "living conditions",
            back: "The circumstances affecting the way people live, such as housing, sanitation, safety, and access to resources.",
            mediaUrl: null,
            deckId: "hpSL740Zl-Mzqo4m6m1mY",
            createdAt: "2025-07-04T07:14:34.508007Z",
          },
          {
            id: "0bm4IYfSMc6_RuTeghTIR",
            front: "pedestrian crossing",
            back: "A marked area on a road where people are allowed to walk across, usually with priority over vehicles.",
            mediaUrl: null,
            deckId: "hpSL740Zl-Mzqo4m6m1mY",
            createdAt: "2025-07-04T07:14:34.508007Z",
          },
          {
            id: "cQMi582DBFxscg9dkG3h2",
            front: "rush hour",
            back: "The busiest time of day on the roads and public transport, typically when people are traveling to or from work.",
            mediaUrl: null,
            deckId: "hpSL740Zl-Mzqo4m6m1mY",
            createdAt: "2025-07-04T07:14:34.508007Z",
          },
          {
            id: "bQ3VKkGQRIeV-v4FwwZ39",
            front: "traffic jam",
            back: "A situation where vehicles are unable to move freely because of heavy congestion.",
            mediaUrl: null,
            deckId: "hpSL740Zl-Mzqo4m6m1mY",
            createdAt: "2025-07-04T07:14:34.508007Z",
          },
          {
            id: "fSE9xUjddVhBDp4ww_6ZE",
            front: "traffic lights",
            back: "Electrically operated signals at road intersections that control the flow of traffic using red, yellow, and green lights.",
            mediaUrl: null,
            deckId: "hpSL740Zl-Mzqo4m6m1mY",
            createdAt: "2025-07-04T07:14:34.508007Z",
          },
          {
            id: "7s2j7dYimr7DA9kX8zMp_",
            front: "workplace",
            back: "The location or environment where a person performs their job or profession.",
            mediaUrl: null,
            deckId: "hpSL740Zl-Mzqo4m6m1mY",
            createdAt: "2025-07-04T07:14:34.508007Z",
          },
          {
            id: "W1ja-HOPa9GEduqBY2aZW",
            front:
              "The location or environment where a person performs their job or profession.",
            back: "The location or environment where a person performs their job or profession.",
            mediaUrl: null,
            deckId: "hpSL740Zl-Mzqo4m6m1mY",
            createdAt: "2025-07-04T07:14:34.508007Z",
          },
          {
            id: "LD3a4SWEnm1t5vacCjAX3",
            front:
              "The location or environment where a person performs their job or profession.",
            back: "The location or environment where a person performs their job or profession.",
            mediaUrl: null,
            deckId: "hpSL740Zl-Mzqo4m6m1mY",
            createdAt: "2025-07-04T07:14:34.508007Z",
          },
          {
            id: "IOpGx6ThhkIAH9TQ0kCS6",
            front:
              "The location or environment where a person performs their job or profession.",
            back: "The location or environment where a person performs their job or profession.",
            mediaUrl: null,
            deckId: "hpSL740Zl-Mzqo4m6m1mY",
            createdAt: "2025-07-04T07:14:34.508007Z",
          },
        ],
        isSubscribed: true,
      };
      break;
  }

  if (!deck) {
    return new Response("Lesson Not Found", { status: 404 });
  }

  return new Response(JSON.stringify(deck), {
    headers: { "Content-Type": "application/json" },
  });
};

export const PATCH: RequestHandler = async () => {
  logger.warn("HIT PUT ENDPOINT");
  return new Response(null, { status: 204 });
};

export const DELETE: RequestHandler = async () => {
  logger.warn("HIT DELETE ENDPOINT");
  return new Response(null, { status: 204 });
};
