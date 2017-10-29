;;; Mendel's First Law

(def AA
  "Homozygous dominant"
  [true true])
(def Aa
  "Heterozygous"
  [true false])
(def aa
  "Homozygous recessive"
  [false false])

(defn cross [a b]
  (let [cases (for [x a y b]
                (or x y))
        n-dominant (->> cases (filter identity) count)
        n (count cases)]
    (/ n-dominant n)))

(def prob-dominant
  (let [cases [AA Aa aa]]
    ; Use nested `for` if need to keep the stucture
    (for [r (range (count cases))
          c (range (inc r))]
      (cross (cases r) (cases c)))))

(defn sample-without-replacement [ks]
  (let [total (apply + ks)
        total2 (dec total)]
    (for [r (range (count ks))
          :let [prob-r-1 (/ (ks r) total)
                prob-r-2 (/ (ks r) total2)]]
      (->
        (for [c (range r)]
          (+ (* prob-r-1 (/ (ks c) total2))
             (* (/ (ks c) total) prob-r-2)))
        vec
        (conj (* prob-r-1 (- prob-r-2 (/ 1 total2))))))))

(defn mendel [AA Aa aa]
  (let [prob-select (-> [AA Aa aa]
                        sample-without-replacement
                        flatten)]
    (->> (map * prob-select prob-dominant) (apply +) float)))
