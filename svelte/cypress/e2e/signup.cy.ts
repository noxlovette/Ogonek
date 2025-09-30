describe("signup page", () => {
  beforeEach(() => {
    cy.visit("/");
  });
  it("the user can sign up", () => {
    cy.dataCy("signup-button").click();
    cy.dataCy("name-field").type("danya");
    cy.dataCy("username-field").type("dev_teacher1");
    cy.dataCy("email-field").type("danila@ogonek.app");
    cy.get("input[name='pass']").type("blablabalba");
    cy.get("input[name='confirmPassword']").type("blablabalba{enter}");

    cy.url().should("include", "/login");
  });

  it("the user can switch to login", () => {
    cy.dataCy("login-button").click();
    cy.url().should("include", "/login");
  });

  it("passwords don't match", () => {
    cy.dataCy("signup-button").click();

    cy.dataCy("name-field").type("danya");
    cy.dataCy("username-field").type("dev_teacher1");
    cy.dataCy("email-field").type("danila@ogonek.app");
    cy.get("input[name='pass']").type("blablabalba");
    cy.get("input[name='confirmPassword']").type("fefe{enter}");

    cy.url().should("include", "/signup");
  });
});
