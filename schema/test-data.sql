INSERT INTO assignments(
  id,
  name,
  role_name,
  description,
  git_url
  ) VALUES (
    1,
    'Ballons',
    'CEO',
    'This is the hardest assignment in existance, good luck bruv',
    'potato.git'
  );

INSERT INTO applicants(
  id,
  name,
  assignment_id,
  git_url
  ) VALUES (
    'e9231914-a669-4213-92e5-652af54507bc',
    'Josef Elrefai',
    1,
    'some git url here'
  );