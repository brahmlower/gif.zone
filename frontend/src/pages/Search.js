
import React, { Component } from 'react'
import { Card, Button, Input, Segment, Form } from 'semantic-ui-react'
import { GifCard } from '../components/GifCard'
import { SiteMenu } from '../components/Menu'

const defaultQuery = {
  captions: 'Any',
  ftype: 'Any',
  labels: [],
  value: ''
}

class PageSearch extends Component {
  constructor (props) {
    super(props)
    this.state = {
      display_results: false,
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
            gifs: result,
            display_results: true
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
    let num_results = (this.state.display_results) ? (<p> There are { this.state.gifs.length } results </p>) : null
    return (
      <div>
        <SiteMenu location={ this.props.location }/>
        <SearchForm onSearch={ this.handleSearch } />
        <hr />
        { num_results }
        <Card.Group itemsPerRow={ 5 } children={ this.gifCardList() } doubling={ true } />
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
        <Input placeholder='Gif search' onChange={ this.handleValueChange } />
      </Form.Field>
    )
  }
}

class CaptionSelection extends Component {
  constructor (props) {
    super(props)
    this.changeCallback = props.onChange
    this.handleItemClick = this.handleItemClick.bind(this)
    this.state = { activeItem: 'Any' }
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
            value='Any'
            active={activeItem === 'Any'}
            onClick={this.handleItemClick}>Any</Button>
          <Button
            size='small'
            value='Yes'
            active={activeItem === 'Yes'}
            onClick={this.handleItemClick}>Yes</Button>
          <Button
            size='small'
            value='No'
            active={activeItem === 'No'}
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
    this.state = { activeItem: 'Any' }
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
            value='Any'
            active={activeItem === 'Any'}
            onClick={this.handleItemClick}>Any</Button>
          <Button
            size='small'
            value='Gif'
            active={activeItem === 'Gif'}
            onClick={this.handleItemClick}>Gif</Button>
          <Button
            size='small'
            value='Webm'
            active={activeItem === 'Webm'}
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
      <Segment>
        <Form>
          <SearchInput onChange={this.handleValueChange} />
          {/* <CaptionSelection onChange={this.handleCaptionChange} />
          <TypeSelection onChange={this.handleTypeChange}/> */}
          <div>
            <Button primary onClick={this.handleSubmit}> Search </Button>
          </div>
        </Form>
      </Segment>
    )
  }
}

export { PageSearch }
