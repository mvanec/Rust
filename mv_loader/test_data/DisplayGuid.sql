-- SELECT ProjectId FROM Projects;
SELECT
  substr(hguid, 1, 2) || substr(hguid, 3, 2) 
    || substr(hguid, 5, 2) || substr(hguid, 7, 2) || '-'
  || substr(hguid, 9, 2) || substr(hguid, 11, 2) || '-'
  || substr(hguid, 13, 2) || substr(hguid, 15, 2) || '-'
  || substr(hguid, 17, 4) || '-'
  || substr(hguid, 21, 12)
AS ProjectId,
ProjectName
FROM (SELECT hex(ProjectId) AS hguid, ProjectName FROM Projects)