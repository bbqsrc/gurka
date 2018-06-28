Feature: User
  Scenario: Create user
    Given a person named John
    When John creates an account
    Then an account named John is created

  Scenario: Edit User
    Given a user logged in named John
    When John edits a property of their account
    Then John's account is updated
  
  Scenario: Read user
    Given a user logged in named Jane
    And an existing user named John
    When Jane accesses John's user profile
    Then John's public profile is shown

  Scenario: Delete user
    Given a user logged in as an administrator
    When an account named John is selected for deletion
    Then John's account no longer exists