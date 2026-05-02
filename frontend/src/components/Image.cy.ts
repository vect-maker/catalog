/// <reference types="cypress" />

import Image from './Image.vue'
import { createPinia } from 'pinia'

describe('<Image />', () => {
  it('renders', () => {
    // see: https://on.cypress.io/mounting-vue

    cy.mount(Image, {
      global: {
        plugins: [createPinia()]
      }
    })
  })
})