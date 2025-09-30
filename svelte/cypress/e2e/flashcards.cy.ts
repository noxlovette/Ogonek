describe("flashcard pages teacher", () => {
  beforeEach(() => {
    cy.visit("/t/dashboard");
  });
  it("teacher can access flashcards", () => {
    cy.dataCy("sidebar-flashcards").first().click();

    cy.url().should("include", "/flashcards");
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
