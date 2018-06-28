Feature: Feature
  Background:
    Given a logged in user named John
    And a project named "Example Project" owned by John

  Scenario: Create feature
    When a feature named "Example Feature" is submitted
    Then a feature named "Example Feature" is created in the project named "Example Project"

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
  