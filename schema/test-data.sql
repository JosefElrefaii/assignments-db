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

INSERT INTO applications(
  id,
  name,
  assignment_id,
  email,
  git_url
  ) VALUES (
    'e9231914-a669-4213-92e5-652af54507bc',
    'Josef Elrefai',
    1,
    'josef.elrefai@hotmail.com',
    'some-git-url-here.git'
  );