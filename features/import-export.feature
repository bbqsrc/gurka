Feature: Import/Export project
  Background:
    Given a logged in user named John
    And a project named "Import/Export Test" owned by John
    And the feature below:
      """
      Feature: Import/Export Feature

        Scenario: Docstring Test
          Given this is a docstring
          When I parse it
          Then I hope it parses nicely
      """

  Scenario: Import a feature into a project
    When the provided feature is imported into the project named "Import/Export Test"
    And a feature named "Import/Export Feature" is queried in the project named "Import/Export Test"
    Then the feature named "Import/Export Feature" is shown

  Scenario: Export all features as .feature files
    Given the provided feature has been imported into the project named "Import/Export Test"
    When all features of the project named "Import/Export Test" are requested to be exported
    Then a URL to a downloadable archive of .feature files is shown
