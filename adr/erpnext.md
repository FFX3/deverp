Issue: ERPs are a complicated piece of software, many needs devloppers have that are solved by ERPs are not unique to them and are shared with many other business. Only minor adjustments are needed to make the ERP tailor made for developers. I'm thus looking at solutions like ERPNext that would provide a ready made ERP that we can then customize to make it our own

Decision: Use of "low code tool" ERPNext to speed up development of DEVerp using mysql

Status: Decided

Group: Framework

Assumptions: 
    - ERPnext applications will be easy to update
    - ERPnext applications will be easy to deploy to production
    - Using the fappe framework we'll be able to deploy new instances using bash scripts
    - Use DigitalOcean we'll be able to create new servers with a ERPnext instance deployed
    - Our deployment script woud be able to add our custom features
    - Backups should be straight forward

Constraints: 
    - Use of mysql (postgres support is still in beta and I've personnaly ran into bugs using it)
    - Use of python

Positions: I've considerd Odoo, I find it comes very bloated out of the box and the code is very confusing, the documentation is a lot less straight to the point and the project. The way the project handles licencing is very complicated, and after diging around for a few days I still don't know how to properly go about creating custom features.

Argument: ERPnext has a great cli tool called bench that uses git to download and install "apps", ERPNext is a "low code" tool but not a "no code" tools. the bench cli simply creates boilerplate and manages database migrations similar to how Laravel's artisan does. Along with the fact that the documentaion is very comprehensive means that we're able to jump into the python code when needed, and since bench uses git repos to download "apps" any modifications made to the python code of an app will be persisted if they are pushed to the repo.

Implications: The use of python and mysql are imposed. debatably I could use postgres if I decide I want to help the ERPNext project bring their postgres support out of beta. We also have to use a MVC architecture and have to use the bench cli to interact with the database.

Related decisions: Use of mysql instead of postgres.

Related requirements: 
    - Get an MVP out within 3 months (start of october 2024)
    - Use of common development tools an flows
    - No restriction on my ability to use software development abilites

Related artifacts: None

Related principles: Code is the best way to tell a system what is should do.

Notes: If we really want to use postgres to create a dashboard application to manage users of the ERP we can use foreign data wrappers
