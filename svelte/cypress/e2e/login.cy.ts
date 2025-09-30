describe("login page", () => {
  beforeEach(() => {
    cy.visit("/");
  });
  it("the user can log in", () => {
    cy.dataCy("login-button").click();
    cy.dataCy("username-field").type("dev_teacher1");
    cy.get("input[name='pass']").type("blablabalba{enter}");

    cy.url().should("include", "/dashboard");
  });

  it("the user can switch to sign up", () => {
    cy.dataCy("signup-button").click();
    cy.url().should("include", "/signup");
  });
});
