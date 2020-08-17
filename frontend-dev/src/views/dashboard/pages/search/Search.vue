<template>
  <v-container
    id="search"
    fluid
    tag="section"
  >
    <download
      :display="dialog"
      @closed="dialog = false"
    />
    <base-material-snackbar
      v-model="snackbar"
      type=""
      top
      left
      color="green"
    >
      Downloading <span class="font-weight-bold">&nbsp;{{ snackbar_title }}&nbsp;</span>
    </base-material-snackbar>

    <v-row
      justify="center"
    >
      <v-col
        cols="12"
        md="8"
      >
        <base-material-card
          color="blue"
          class="px-5 py-3"
        >
          <template v-slot:heading>
            <div class="display-3 font-weight-light">
              Search
            </div>
          </template>
          <v-card-text>
            <v-form>
              <v-container>
                <v-row>
                  <v-col
                    cols="12"
                    md="8"
                  >
                    <v-text-field
                      v-model="search_val"
                      label="Search"
                      required
                    />
                  </v-col>
                  <v-col
                    class="d-flex"
                    cols="12"
                    sm="4"
                  >
                    <v-select
                      v-model="entertainment_type"
                      :items="items"
                      item-text="type"
                      item-value="value"
                      label="Entertainment Type"
                    />
                  </v-col>
                  <v-btn
                    :loading="searching"
                    color="success"
                    class="mr-4"
                    @click="get_search"
                  >
                    Search
                  </v-btn>
                </v-row>
              </v-container>
            </v-form>
          </v-card-text>
        </base-material-card>
      </v-col>
    </v-row>
    <v-row>
      <v-col
        v-for="(title, i) in titles"
        :key="i"
        cols="12"
        md="4"
      >
        <template>
          <v-card
            class="mx-auto"
            max-width="400"
          >
            <v-img
              class="white--text align-end"
              :src="title.Poster !== 'N/A' ? title.Poster : unknownPoster"
            />

            <v-card-subtitle class="pb-0">
              <v-icon>{{ title.Type === 'movie' ? 'mdi-movie' : 'mdi-television' }}</v-icon>
            </v-card-subtitle>

            <v-card-text class="text--primary">
              <div>{{ title.Title }}</div>
              <div>{{ title.Year }}</div>
            </v-card-text>

            <v-card-actions>
              <v-btn
                color="orange"
                text
                @click="download(title, $event)"
              >
                <v-icon>mdi-cloud-download</v-icon><span style="padding-left: 10px;">Download</span>
              </v-btn>
            </v-card-actions>
          </v-card>
        </template>
      </v-col>
    </v-row>
  </v-container>
</template>

<script>
  import axios from 'axios'
  import qs from 'querystring'
  import Download from './Download'

  export default {
    name: 'Search',
    components: {
      download: Download,
    },
    data: () => ({
      dialog: false,
      snackbar: false,
      snackbar_title: '',
      unknownPoster: require('@/assets/Unknown.png'),
      items: [
        {
          type: 'Any',
          value: '',
        },
        {
          type: 'Movies',
          value: 'movie',
        },
        {
          type: 'TV',
          value: 'series',
        },
      ],
      searching: false,
      search_val: '',
      entertainment_type: '',
      titles: [],
      loading: false,
    }),

    methods: {
      download (title, event) {
        // TODO: This is for queuing and automatic donwloading
        // this.snackbar = true
        // this.snackbar_title = title.Title + ' (' + title.Year + ')'
      },
      get_search () {
        this.searching = true
        const requestBody = {
          search: this.search_val,
          entertainment_type: this.entertainment_type,
        }

        const config = {
          headers: {
            'Content-Type': 'application/x-www-form-urlencoded',
          },
        }

        axios.post('/search', qs.stringify(requestBody), config)
          .then((result) => {
            this.titles = result.data.Search
            // TODO: Create page results from number of results
            console.log(result.data)
            this.searching = false
          })
          .catch((err) => {
            console.log(err)
          })
      },
    },
  }
</script>
