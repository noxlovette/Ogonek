describe("flashcard pages teacher", () => {
  beforeEach(() => {
    cy.visit("/t/dashboard");
  });
  it("teacher can access flashcards", () => {
    cy.dataCy("sidebar-flashcards").first().click();

    cy.url().should("include", "/flashcards");
  });

  it("teacher can add a new card", () => {
    cy.visit("/t/flashcards/oPTQY_b27XKL7wUgvGbw_");
    cy.dataCy("edit-button").click();
    cy.dataCy("new-card").click();
    cy.dataCy("save-button").click();
    cy.url().should("match", /\/flashcards\/\w+/);
  });
});

describe("flashcard pages student", () => {
  beforeEach(() => {
    cy.visit("/s/dashboard");
  });
  it("student can access flashcards", () => {
    cy.dataCy("sidebar-flashcards").first().click();

    cy.url().should("include", "/flashcards");
  });

  it("student can access one deck", () => {
    cy.dataCy("sidebar-flashcards").first().click();
    cy.dataCy("deck-card").first().click();
    cy.url().should("match", /\/flashcards\/\w+/);
  });
});
