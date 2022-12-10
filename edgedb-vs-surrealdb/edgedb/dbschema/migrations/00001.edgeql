CREATE MIGRATION m1zoh66uubvomckprf6mh6macmvhk2kfloyyiicfxqwjpwxdrr2xwa
    ONTO initial
{
  CREATE FUTURE nonrecursive_access_policies;
  CREATE TYPE default::Repository {
      CREATE REQUIRED PROPERTY language -> std::str;
      CREATE REQUIRED PROPERTY name -> std::str {
          CREATE CONSTRAINT std::exclusive;
      };
      CREATE REQUIRED PROPERTY url -> std::str;
  };
};
