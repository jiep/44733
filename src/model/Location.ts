export class Location {
  id: number
  address: string
  city: string
  province: string
  series: Array<number>
  phone: string

  constructor(
    id: number,
    address: string,
    city: string,
    province: string,
    series: Array<number>,
    phone: string
  ) {
    this.id = id
    this.address = address
    this.city = city
    this.province = province
    this.series = series
    this.phone = phone
  }
}
