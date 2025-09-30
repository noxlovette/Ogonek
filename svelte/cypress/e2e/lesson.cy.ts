describe("lesson pages teacher", () => {
  beforeEach(() => {
    cy.visit("/t/dashboard");
  });
  it("teacher can access lessons", () => {
    cy.dataCy("sidebar-lessons").first().click();

    cy.url().should("include", "/lessons");
  });
});

describe("lesson pages student", () => {
  beforeEach(() => {
    cy.visit("/s/dashboard");
  });
  it("student can access lessons", () => {
    cy.dataCy("sidebar-lessons").first().click();

    cy.url().should("include", "/lessons");
  });

  it("student can access one lesson", () => {
    cy.dataCy("sidebar-lessons").first().click();
    cy.dataCy("lesson-card").first().click();
    cy.url().should("match", /\/lessons\/\w+/);
  });
});
