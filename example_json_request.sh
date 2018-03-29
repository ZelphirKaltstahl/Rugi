# #########################
# # TRYING A POST REQUEST #
# #########################
# curl \
#     --dump-header - \
#     --header "Content-Type: application/json" \
#     -X POST \
#     --data '{
#   "sfen": "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1",
#   "move_in_position": {
#     "Normal": {
#       "from": {
#         "file": 0,
#         "rank": 0,
#         "piece": {
#           "kind": "Lance",
#           "owner": "Sente",
#           "promoted": false
#         }
#       },
#       "to": {
#         "file": 0,
#         "rank": 1,
#         "piece":null
#       },
#       "promote":false
#     }
#   }
# }' \
#     http://localhost:8000/try_move_with_position

########################
# TRYING A GET REQUEST #
########################
printf "%s\n\n" "The following request SHOULD SUCCEED and the move should be INVALID:"
curl\
    -G\
    -v\
    "http://localhost:8000/try_move_with_position"\
    --data-urlencode 'json={
  "sfen": "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1",
  "move_in_position": {
    "Normal": {
      "from": {
        "file": 0,
        "rank": 0,
        "piece": {
          "kind": "Lance",
          "owner": "Sente",
          "promoted": false
        }
      },
      "to": {
        "file": 0,
        "rank": 1,
        "piece":null
      },
      "promote":false
    }
  }
}'
printf "\n"

########################
# TRYING A GET REQUEST #
########################
printf "%s\n\n" "The following request SHOULD SUCCEED and the move should be VALID:"
curl\
    -G\
    -v\
    "http://localhost:8000/try_move_with_position"\
    --data-urlencode 'json={
  "sfen": "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1",
  "move_in_position": {
    "Normal": {
      "from": {
        "file": 8,
        "rank": 8,
        "piece": {
          "kind": "Lance",
          "owner": "Sente",
          "promoted": false
        }
      },
      "to": {
        "file": 8,
        "rank": 7,
        "piece":null
      },
      "promote":false
    }
  }
}'
printf "\n"

printf "%s\n\n" "The following request SHOULD FAIL:"
curl\
    -G\
    -v\
    "http://localhost:8000/try_move_with_position"\
    --data-urlencode 'json={
  "sfen": "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1",
  "move_in_position": {
    "Normal": {
      "from": {
        "file": 0,
        "rank": 0,
        "piece": {
          "kind": "Lances",
          "owner": "Sente",
          "promoted": false
        }
      },
      "to": {
        "file": 0,
        "rank": 1,
        "piece":null
      },
      "promote":false
    }
  }
}'
printf "\n"
