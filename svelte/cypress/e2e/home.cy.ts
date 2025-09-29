describe("home page", () => {
  beforeEach(() => {
    cy.visit("/");
  });
  it("there is a hero", () => {
    cy.dataCy("hero");
  });
});
