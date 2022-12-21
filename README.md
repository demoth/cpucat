# CPU cat

`cat` program with coloring based on the cpu usage.

Intended for use with long-running processes (like `mvn install`) to highlight hot places.

Example use:

    mvn clean install | cpucat
