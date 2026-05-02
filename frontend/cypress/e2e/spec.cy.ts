/// <reference types="cypress" />

describe('Go to home', () => {
  beforeEach(() => {
    cy.intercept('GET', '**/health/ready').as('backendReady')
    cy.visit('http://192.168.1.23:5173')
  })

  it('Opens search modal after backend is ready', () => {
    cy.wait('@backendReady')

    cy.get('[data-cy="nav-search-btn"]').click()
  })
})