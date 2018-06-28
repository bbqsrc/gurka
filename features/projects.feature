Feature: Projects
  Background:
    Given a logged in user named John

  Scenario: Create project
    When a project named "Example Project" is submitted
    Then a project named "Example Project" is created
    And a project named "Example Project" is owned by John

  Scenario: Read project
    Given a project named "Example Project"
    When a project named "Example Project" is selected
    Then a project named "Example Project" is shown

  Scenario: Edit project
    Given a project named "Example Project"
    When a project named "Example Project" is renamed to "Another Name"
    Then a project named "Another Name" is shown
    And a project named "Example Project" no longer exists

  Scenario: Delete project
    Given a project named "Example Project"
    When a project named "Example Project" is selected for deletion
    Then a project named "Example Project" no longer exists