describe("task pages teacher", () => {
  beforeEach(() => {
    cy.visit("/t/dashboard");
  });
  it("teacher can access tasks", () => {
    cy.dataCy("sidebar-tasks").first().click();

    cy.url().should("include", "/tasks");
  });
});

describe("task pages student", () => {
  beforeEach(() => {
    cy.visit("/s/dashboard");
  });
  it("student can access tasks", () => {
    cy.dataCy("sidebar-tasks").first().click();

    cy.url().should("include", "/tasks");
  });

  it("student can access one task", () => {
    cy.dataCy("task-card").first().click();
    cy.url().should("match", /\/tasks\/\w+/);
  });
});
