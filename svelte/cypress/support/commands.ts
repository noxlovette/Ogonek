// load the global Cypress types
/// <reference types="cypress" />

/**
 * Adds custom command "cy.dataCy" to the global "cy" object
 *
 * @example cy.dataCy('greeting')
 */
Cypress.Commands.add("dataCy", (value) => cy.get(`[data-cy=${value}]`));

// the types for cy.dataCy will be defined in "index.d.ts" file
