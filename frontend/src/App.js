import React, { Component } from 'react';
import { Card, Container } from 'semantic-ui-react';
import { GifCard } from './GifCard';
import { SearchForm } from './Search';
import { SiteFooter } from './SiteFooter';
import './App.css';

class App extends Component {
  constructor(props) {
    super(props);
    this.state = {
      gifs: [
        {
          title: "Patrick Construction 1",
          slug: "patrick-construction.gif",
          views: "15k",
          type: "webm",
          description: "This is the first of many gifs!"
        },
        {
          title: "Patrick Construction 2",
          slug: "patrick-construction.gif",
          views: "15k",
          type: "gif"
        },
        {
          title: "Patrick Construction 3",
          slug: "patrick-construction.gif",
          views: "15k",
          type: "webm"
        },
        {
          title: "Patrick Construction 4",
          slug: "patrick-construction.gif",
          views: "15k",
          type: "gif"
        },
        {
          title: "Patrick Construction 5",
          slug: "patrick-construction.gif",
          views: "15k",
          type: "webm"
        },
        {
          title: "Patrick Construction 6",
          slug: "patrick-construction.gif",
          views: "15k",
          type: "gif"
        },
        {
          title: "Patrick Construction 7",
          slug: "patrick-construction.gif",
          views: "15k",
          type: "webm"
        }
      ]
    }
    this.gifCardList = this.gifCardList.bind(this);
  }

  gifCardList() {
    return this.state.gifs.map((gif) => {
      return (<GifCard {...gif} />)
    });
  }

  render() {
    return (
      <div className="App">
        <Container>
          <h1> Gif zone </h1>
          <SearchForm />
          <hr />
          <Card.Group itemsPerRow={5} children={this.gifCardList()} />
          <SiteFooter />
        </Container>
      </div>
    );
  }
}

export default App;
