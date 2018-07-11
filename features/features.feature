Feature: Feature

  Background:
    Given a logged in user named John
    And a project with slug "example-project" owned by John

  Scenario: Create feature
    When a feature with slug "example-feature" is submitted
    Then a feature with slug "example-feature" is created in the project with slug "example-project"

  Scenario: Read feature
    When a feature named "Example Feature" is selected
    Then a feature named "Example Feature" is shown

  Scenario: Edit feature
    When a feature named "Example Feature" is renamed to "A Better Name"
    Then a feature named "A Better Name" is shown
    And a feature named "Example Feature" no longer exists

  Scenario: Delete feature
    When a feature named "Example Feature" is selected for deletion
    Then a feature named "Example Feature" no longer exists              