install htop:
  pkg.installed:
    - pkgs:
      - htop

hello test:
    file.managed:
        - name: /tmp/htop
        - contents: |
            #!/bin/bash
            echo "Hello World"
        - owner: root
        - group: root
        - mode: 755
        - require:
          - install htop

