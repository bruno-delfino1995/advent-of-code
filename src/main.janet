(defn list []
  (print "List is not implemented"))

(defn solve [puzzle]
  (print "Solve is not implemented"))

(defn main [& args]
  (case (in args 1)
    "list" (list)
    "solve" (solve (in args 2))
    (print "Unknown command")))
