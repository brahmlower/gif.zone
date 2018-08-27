
import React, { Component } from 'react'
import { Card, Button, Input, Segment, Form } from 'semantic-ui-react'
import { GifCard } from '../GifCard'

const defaultQuery = {
  captions: 'any',
  ftype: 'any',
  labels: [],
  value: ''
}

class PageSearch extends Component {
  constructor (props) {
    super(props)
    this.state = {
      gifs: []
    }
    this.gifCardList = this.gifCardList.bind(this)
    this.handleSearch = this.handleSearch.bind(this)
  }

  componentDidMount () {
    this.handleSearch(defaultQuery)
  }

  handleSearch (query) {
    let args = {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json; charset=utf-8'
      },
      body: JSON.stringify(query)
    }
    fetch('/api/search', args)
      .then(res => res.json())
      .then(
        (result) => {
          this.setState({
            gifs: result
          })
        },
        (error) => {
          console.log('api loading error!')
          console.log(error)
          this.setState({
            isLoaded: true,
            error
          })
        }
      )
  }

  gifCardList () {
    return this.state.gifs.map((gif, i) => {
      return (<GifCard key={i} {...gif} />)
    })
  }

  render () {
    return (
      <div>
        <SearchForm onSearch={this.handleSearch} />
        <hr />
        <Card.Group itemsPerRow={5} children={this.gifCardList()} />
      </div>
    )
  }
}

class SearchInput extends Component {
  constructor (props) {
    super(props)
    this.changeCallback = props.onChange
    this.handleValueChange = this.handleValueChange.bind(this)
  }

  handleValueChange (event) {
    this.changeCallback(event.target.value)
  }

  render () {
    return (
      <Form.Field>
        <Input placeholder='Gif search' onChange={this.handleValueChange} />
      </Form.Field>
    )
  }
}

class CaptionSelection extends Component {
  constructor (props) {
    super(props)
    this.changeCallback = props.onChange
    this.handleItemClick = this.handleItemClick.bind(this)
    this.state = { activeItem: 'any' }
  }

  handleItemClick (e, { value }) {
    this.setState({ activeItem: value })
    this.changeCallback(value)
  }

  render () {
    const { activeItem } = this.state
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
  constructor (props) {
    super(props)
    this.changeCallback = props.onChange
    this.state = { activeItem: 'any' }
    this.handleItemClick = this.handleItemClick.bind(this)
  }

  handleItemClick (e, { value }) {
    this.setState({ activeItem: value })
    this.changeCallback(value)
  }

  render () {
    const { activeItem } = this.state
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
  constructor (props) {
    super(props)
    this.submitCallback = props.onSearch
    this.handleSubmit = this.handleSubmit.bind(this)
    this.handleValueChange = this.handleValueChange.bind(this)
    this.handleCaptionChange = this.handleCaptionChange.bind(this)
    this.handleTypeChange = this.handleTypeChange.bind(this)
    this.state = {
      query: defaultQuery
    }
  }

  handleSubmit (_) {
    this.submitCallback(this.state.query)
  }

  handleValueChange (value) {
    let state = this.state
    state.query.value = value
    this.setState(state)
  }

  handleCaptionChange (value) {
    let state = this.state
    state.query.captions = value
    this.setState(state)
  }

  handleTypeChange (value) {
    let state = this.state
    state.query.ftype = value
    this.setState(state)
  }

  render () {
    return (
      <Segment style={{textAlign: 'left'}}>
        <Form>
          <SearchInput onChange={this.handleValueChange} />
          <div className='row'>
            <CaptionSelection onChange={this.handleCaptionChange} />
            <TypeSelection onChange={this.handleTypeChange}/>
            <div>
              <Button primary onClick={this.handleSubmit}> Search </Button>
            </div>
          </div>
        </Form>
      </Segment>
    )
  }
}

export { PageSearch }
