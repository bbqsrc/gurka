Feature: Projects

  Background:
    Given a logged in user named John

  Scenario: Create project
    When a project with slug "example-project" is submitted
    Then a project with slug "example-project" is created
    And a project with slug "example-project" is owned by John

  Scenario: Read project
    Given a project with slug "example-project"
    When a project with slug "example-project" is selected
    Then a project with slug "example-project" is shown

  Scenario: Edit project
    Given a project with slug "example-project"
    When a project with slug "example-project" is renamed to "another-name"
    Then a project with slug "another-name" is shown
    And a project with slug "example-project" no longer exists

  Scenario: Delete project
    Given a project with slug "example-project"
    When a project with slug "example-project" is selected for deletion
    Then a project with slug "example-project" no longer exists