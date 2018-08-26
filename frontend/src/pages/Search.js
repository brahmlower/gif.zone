import React, { Component } from 'react';
import { Card, Button, Input, Segment, Form } from 'semantic-ui-react';
import { GifCard } from '../GifCard';

class PageSearch extends Component {
  constructor(props) {
    super(props);
    this.state = {
      gifs: []
    }
    this.gifCardList = this.gifCardList.bind(this);
  }

  componentDidMount() {
    fetch("/api/gif")
      .then(res => res.json())
      .then(
        (result) => {
          this.setState({
            gifs: result
          });
        },
        // Note: it's important to handle errors here
        // instead of a catch() block so that we don't swallow
        // exceptions from actual bugs in components.
        (error) => {
          console.log('api loading error!')
          console.log(error)
          this.setState({
            isLoaded: true,
            error
          });
        }
      )
  }

  gifCardList() {
    return this.state.gifs.map((gif) => {
      return (<GifCard {...gif} />)
    });
  }

  render() {
    return (
      <div>
        <SearchForm />
        <hr />
        <Card.Group itemsPerRow={5} children={this.gifCardList()} />
      </div>
    )
  }
}

class SearchInput extends Component {
  render() {
    return (
      <Form.Field>
        <Input placeholder='Gif search'>{this.props.value}</Input>
      </Form.Field>
    )
  }
}

class CaptionSelection extends Component {
  constructor(props) {
    super(props);
    this.state = { activeItem: 'any' }
  }
  handleItemClick = (e, { value }) => this.setState({ activeItem: value })
  // todo: needs onChange callback
  render() {
    return (
      <Form.Field>
        <label>Has Caption</label>
        <Button.Group>
          <Button
            size='small'
            value='any'
            active={activeItem === 'any'}
            onClick={this.handleItemClick}>Any</Button>
          <Button
            size='small'
            value='yes'
            active={activeItem === 'yes'}
            onClick={this.handleItemClick}>Yes</Button>
          <Button
            size='small'
            value='no'
            active={activeItem === 'no'}
            onClick={this.handleItemClick}>No</Button>
        </Button.Group>
      </Form.Field>
    )
  }
}

class TypeSelection extends Component {
  constructor(props) {
    super(props);
    this.state = { activeItem: 'any' }
  }
  handleItemClick = (e, { value }) => this.setState({ activeItem: value })
  // todo: needs onChange callback
  render() {
    const { activeItem } = this.state;
    return (
      <Form.Field>
        <label>Resource Type</label>
        <Button.Group>
          <Button
            size='small'
            value='any'
            active={activeItem === 'any'}
            onClick={this.handleItemClick}>Any</Button>
          <Button
            size='small'
            value='gif'
            active={activeItem === 'gif'}
            onClick={this.handleItemClick}>Gif</Button>
          <Button
            size='small'
            value='webm'
            active={activeItem === 'webm'}
            onClick={this.handleItemClick}>Webm</Button>
        </Button.Group>
      </Form.Field>
    )
  }
}

class SearchForm extends Component {
  constructor(props) {
    super(props)
    this.state = {
      query: {
        captions: 'any',
        ftype: 'any',
        labels: [],
        value: ''
      }
    }
  }
  render() {
    return (
      <Segment style={{textAlign: 'left'}}>
        <Form>
          <SearchInput />
          <div className="row">
            <CaptionSelection />
            <TypeSelection />
            <div>
              <Button primary> Search </Button>
            </div>
          </div>
        </Form>
      </Segment>
    )
  }
}

export { PageSearch };
